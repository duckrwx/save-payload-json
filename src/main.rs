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
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    caminho_drive: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/adicionar", post(adicionar_registro))
        .route("/api/config", get(obter_config))
        .route("/api/config", post(salvar_config))
        .route("/api/config/verificar", get(verificar_config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("🚀 Servidor rodando em http://localhost:3000");
    
    if webbrowser::open("http://localhost:3000").is_ok() {
        println!("✅ Navegador aberto!");
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Verifica se config existe
async fn verificar_config() -> impl IntoResponse {
    let existe = std::path::Path::new("config.json").exists();
    Json(serde_json::json!({ "configurado": existe }))
}

// Obtém a configuração atual
async fn obter_config() -> impl IntoResponse {
    match fs::read_to_string("config.json") {
        Ok(content) => {
            let config: Config = serde_json::from_str(&content).unwrap();
            (StatusCode::OK, Json(config))
        }
        Err(_) => {
            (StatusCode::NOT_FOUND, Json(Config {
                caminho_drive: String::new()
            }))
        }
    }
}

// Salva nova configuração
async fn salvar_config(Json(config): Json<Config>) -> impl IntoResponse {
    match serde_json::to_string_pretty(&config) {
        Ok(json_str) => {
            if fs::write("config.json", json_str).is_ok() {
                println!("✅ Configuração salva: {}", config.caminho_drive);
                (StatusCode::OK, Json(serde_json::json!({
                    "status": "sucesso",
                    "mensagem": "Configuração salva com sucesso!"
                })))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "status": "erro",
                    "mensagem": "Erro ao salvar configuração"
                })))
            }
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "erro",
                "mensagem": "Erro ao processar configuração"
            })))
        }
    }
}

async fn adicionar_registro(
    Json(dados): Json<TaskManager>,
) -> impl IntoResponse {
    println!("\n🔔 Requisição recebida!");
    
    match dados.processar() {
        Ok(caminho) => {
            println!("✅ Arquivo salvo: {}", caminho);
            (StatusCode::OK, Json(serde_json::json!({
                "status": "sucesso",
                "mensagem": "Registro salvo com sucesso!",
                "caminho": caminho
            })))
        }
        Err(e) => {
            eprintln!("❌ Erro: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "erro",
                "mensagem": format!("Erro ao salvar: {}", e)
            })))
        }
    }
}