mod models;
mod task_manager;
mod error;
mod storage;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
};
use tower_http::services::ServeDir;
use task_manager::TaskManager;
use models::Registro;
use storage::{Storage, MongoStorage};
use mongodb::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use dotenv::dotenv;
use std::env;
use crate::error::AppError;

#[derive(Clone)]
struct AppState {
    storage: Arc<RwLock<Option<Arc<dyn Storage>>>>,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    
    // Cria o estado com storage vazio (será preenchido depois)
    let state = AppState { 
        storage: Arc::new(RwLock::new(None))
    };
    
    // Configura rotas PRIMEIRO (antes de conectar MongoDB)
    let app = Router::new()
        .route("/api/adicionar", post(adicionar_registro))
        .route("/api/health", get(health_check))
        .fallback_service(ServeDir::new("static"))
        .with_state(state.clone());
    
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}")
        .parse()
        .map_err(AppError::AddressParseError)?;
    
    println!("🚀 Servidor iniciando em http://localhost:{port}");
    println!("🌍 Ambiente: {}", env::var("RAILWAY_ENVIRONMENT").unwrap_or_else(|_| "local".to_string()));
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(AppError::IoError)?;
    println!("👂 Escutando em {addr}");
    println!("✅ Health check disponível em /api/health");
    
    // Conecta ao MongoDB em background
    let state_clone = state.clone();
    tokio::spawn(async move {
        match conectar_mongodb().await {
            Ok(storage) => {
                *state_clone.storage.write().await = Some(storage);
                println!("✅ MongoDB conectado com sucesso!");
            }
            Err(e) => {
                eprintln!("❌ Erro ao conectar MongoDB: {}", e);
                eprintln!("⚠️ Servidor continua rodando sem MongoDB");
            }
        }
    });
    
    // Não abre navegador em produção
    if env::var("RAILWAY_ENVIRONMENT").is_err() && 
       env::var("DO_APP_NAME").is_err() {
        if let Err(e) = webbrowser::open(&format!("http://localhost:{port}")) {
            eprintln!("⚠️ Não foi possível abrir o navegador: {e}");
        }
    }
    
    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::IoError(std::io::Error::other(e)))?;
    
    Ok(())
}

async fn conectar_mongodb() -> Result<Arc<dyn Storage>, AppError> {
    let mongodb_uri = env::var("MONGODB_URI")
        .map_err(|_| AppError::ConfigError("MONGODB_URI deve estar definida".into()))?;
    let database_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "registros_db".to_string());
    let collection_name = env::var("COLLECTION_NAME")
        .unwrap_or_else(|_| "registros".to_string());
    
    println!("� Conectando ao MongoDB...");
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .map_err(|e| AppError::ConnectionError(e.to_string()))?;
    
    let database = client.database(&database_name);
    let collection = database.collection::<Registro>(&collection_name);
    
    println!("📦 Database: {database_name}");
    println!("📁 Collection: {collection_name}");

    Ok(Arc::new(MongoStorage::new(collection)))
}

async fn health_check() -> impl IntoResponse {
    // Health check simples - não depende de nada externo
    (StatusCode::OK, Json(serde_json::json!({
        "status": "ok",
        "service": "registros-json",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

async fn adicionar_registro(
    State(state): State<AppState>,
    Json(dados): Json<TaskManager>,
) -> impl IntoResponse {
    println!("\n📝 Nova requisição recebida!");
    println!("   Estado: {:?}", dados.estado);
    println!("   Responsável: {}", dados.responsavel);
    
    // Verifica se MongoDB está conectado
    let storage_guard = state.storage.read().await;
    let storage = match storage_guard.as_ref() {
        Some(s) => s,
        None => {
            eprintln!("❌ MongoDB ainda não conectado");
            return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
                "status": "erro",
                "mensagem": "Banco de dados ainda não está disponível. Tente novamente em alguns segundos."
            })));
        }
    };
    
    match dados.processar(storage.as_ref()).await {
        Ok(mensagem) => {
            println!("✅ Sucesso: {mensagem}");
            (StatusCode::OK, Json(serde_json::json!({
                "status": "sucesso",
                "mensagem": mensagem
            })))
        }
        Err(e) => {
            eprintln!("❌ Erro: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "erro",
                "mensagem": format!("{}", e)
            })))
        }
    }
}