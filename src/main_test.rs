mod models;
mod task_manager;
mod error;

use axum::{
    routing::{get, post},
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
};
use tower_http::services::ServeDir;
use task_manager::TaskManager;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("⚠️  MODO TESTE - Salvando em arquivos JSON locais");
    println!("📁 Pasta: ./registros/");
    
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/adicionar", post(adicionar_registro))
        .route("/api/health", get(health_check));
    
    let port = 3000;
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    
    println!("🚀 Servidor rodando em http://localhost:{}", port);
    
    let _ = webbrowser::open(&format!("http://localhost:{}", port));
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "mode": "file-storage"
    }))
}

async fn adicionar_registro(
    Json(dados): Json<TaskManager>,
) -> impl IntoResponse {
    println!("\n📝 Recebendo registro...");
    
    match dados.processar() {
        Ok(mensagem) => {
            println!("✅ {}", mensagem);
            (StatusCode::OK, Json(serde_json::json!({
                "status": "sucesso",
                "mensagem": mensagem
            })))
        }
        Err(e) => {
            eprintln!("❌ Erro: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "erro",
                "mensagem": format!("Erro: {}", e)
            })))
        }
    }
}