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
use dotenv::dotenv;
use std::env;
use crate::error::AppError;

#[derive(Clone)]
struct AppState {
    storage: Arc<dyn Storage>,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    
    let mongodb_uri = env::var("MONGODB_URI")
        .map_err(|_| AppError::ConfigError("MONGODB_URI deve estar definida no .env".into()))?;
    let database_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "registros_db".to_string());
    let collection_name = env::var("COLLECTION_NAME")
        .unwrap_or_else(|_| "registros".to_string());
    
    println!("🔌 Conectando ao MongoDB...");
    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .map_err(|e| AppError::ConnectionError(e.to_string()))?;
    
    let database = client.database(&database_name);
    let collection = database.collection::<Registro>(&collection_name);
    
    println!("✅ Conectado ao MongoDB!");
    println!("📦 Database: {database_name}");
    println!("📁 Collection: {collection_name}");

    let storage: Arc<dyn Storage> = Arc::new(MongoStorage::new(collection));
    let state = AppState { storage };
    
    // ✅ CORREÇÃO AQUI - fallback_service em vez de nest_service
    let app = Router::new()
        .route("/api/adicionar", post(adicionar_registro))
        .route("/api/health", get(health_check))
        .fallback_service(ServeDir::new("static"))  // ← MUDOU AQUI!
        .with_state(state);
    
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}")
        .parse()
        .map_err(AppError::AddressParseError)?;

    println!("🚀 Servidor rodando em http://localhost:{port}");

    // Não abre navegador em produção
    if env::var("RAILWAY_ENVIRONMENT").is_err() && 
       env::var("DO_APP_NAME").is_err() {
        if let Err(e) = webbrowser::open(&format!("http://localhost:{port}")) {
            eprintln!("⚠️ Não foi possível abrir o navegador: {e}");
        }
    }
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(AppError::IoError)?;
    println!("👂 Escutando em {addr}");
    
    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::IoError(std::io::Error::other(e)))?;
    
    Ok(())
}

async fn health_check() -> impl IntoResponse {
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

    match dados
        .processar(state.storage.as_ref())
        .await {
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
                "mensagem": format!("{e}")
            })))
        }
    }
}