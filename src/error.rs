use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    InvalidData(String),
    ConfigError(String),
    ConnectionError(String),
    DatabaseError(String),
    AddressParseError(std::net::AddrParseError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "Erro de I/O: {e}"),
            Self::SerializationError(e) => write!(f, "Erro de serialização: {e}"),
            Self::InvalidData(msg) => write!(f, "Dados inválidos: {msg}"),
            Self::ConfigError(msg) => write!(f, "Erro de configuração: {msg}"),
            Self::ConnectionError(msg) => write!(f, "Erro de conexão: {msg}"),
            Self::DatabaseError(msg) => write!(f, "Erro no banco de dados: {msg}"),
            Self::AddressParseError(e) => write!(f, "Erro ao parsear endereço: {e}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err)
    }
}