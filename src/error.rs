use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Erro de serialização: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Dados inválidos: {0}")]
    InvalidData(String),
    
    #[error("Erro no caminho: {0}")]
    PathError(String)
}