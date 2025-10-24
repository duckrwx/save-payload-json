use crate::models::{Registro, Estado};
use crate::error::AppError;
use crate::storage::Storage;
use chrono::Local;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskManager {
    pub estado: Estado,
    pub responsavel: String,
    pub payload: String,
}

impl TaskManager {
    /// Processa e salva usando qualquer implementação de Storage
    pub async fn processar(
        self,
        storage: &dyn Storage
    ) -> Result<String, AppError> {
        let regional = self.estado.obter_regional();
        
        let registro = Registro::new(
            self.estado,
            regional,
            self.responsavel,
            Local::now().naive_local().date(),
            self.payload,
        );
        
        storage.salvar(registro).await
    }
}