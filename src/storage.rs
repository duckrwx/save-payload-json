use crate::models::Registro;
use crate::error::AppError;
use async_trait::async_trait;

// ==================== TRAIT GENÉRICA ====================

#[async_trait]
pub trait Storage: Send + Sync {
    async fn salvar(&self, registro: Registro) -> Result<String, AppError>;
}



// ==================== IMPLEMENTAÇÃO COM MONGODB ====================

use mongodb::Collection;

pub struct MongoStorage {
    collection: Collection<Registro>,
}

impl MongoStorage {
    pub fn new(collection: Collection<Registro>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl Storage for MongoStorage {
    async fn salvar(&self, registro: Registro) -> Result<String, AppError> {
        let result = self.collection
            .insert_one(registro)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let id = result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| AppError::DatabaseError("ID não gerado pelo MongoDB".into()))?;
        
        Ok(format!("Registro salvo com ID: {}", id))
    }
}