use crate::models::Registro;
use crate::error::AppError;
use async_trait::async_trait;

// ==================== TRAIT GENÉRICA ====================

#[async_trait]
pub trait Storage: Send + Sync {
    async fn salvar(&self, registro: Registro) -> Result<String, AppError>;
    async fn listar_todos(&self) -> Result<Vec<Registro>, AppError>;
    async fn buscar_por_id(&self, id: &str) -> Result<Option<Registro>, AppError>;
}

// ==================== IMPLEMENTAÇÃO COM ARQUIVOS ====================

pub struct FileStorage {
    base_path: String,
}

impl FileStorage {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
    
    fn gerar_caminho(&self, registro: &Registro) -> String {
        let estado_str = format!("{:?}", registro.estado);
        let regional_str = serde_json::to_string(&registro.regional)
            .unwrap_or_default()
            .trim_matches('"')
            .to_string();
        
        format!(
            "{}/{}/{}/{}.json",
            self.base_path,
            estado_str,
            regional_str,
            registro.data
        )
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn salvar(&self, registro: Registro) -> Result<String, AppError> {
        let caminho = self.gerar_caminho(&registro);
        
        // Cria diretórios
        if let Some(parent) = std::path::Path::new(&caminho).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        // Serializa
        let json = serde_json::to_string_pretty(&registro)?;
        
        // Salva
        tokio::fs::write(&caminho, json).await?;
        
        Ok(format!("Arquivo salvo em: {caminho}"))
    }
    
    async fn listar_todos(&self) -> Result<Vec<Registro>, AppError> {
        // TODO: implementar leitura recursiva de arquivos se necessário
        Ok(vec![])
    }
    
    async fn buscar_por_id(&self, _id: &str) -> Result<Option<Registro>, AppError> {
        // TODO: implementar busca em arquivos se necessário
        Ok(None)
    }
}

// ==================== IMPLEMENTAÇÃO COM MONGODB ====================

use mongodb::Collection;
use mongodb::bson::oid::ObjectId;

pub struct MongoStorage {
    collection: Collection<Registro>,
}

impl MongoStorage {
    pub const fn new(collection: Collection<Registro>) -> Self {
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
        
        Ok(format!("Registro salvo com ID: {id}"))
    }
    
    async fn listar_todos(&self) -> Result<Vec<Registro>, AppError> {
        use futures::stream::TryStreamExt;
        
        let cursor = self.collection
            .find(mongodb::bson::doc! {})
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let registros = cursor
            .try_collect()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(registros)
    }
    
    async fn buscar_por_id(&self, id: &str) -> Result<Option<Registro>, AppError> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|e| AppError::InvalidData(format!("ID inválido: {e}")))?;
        
        let filter = mongodb::bson::doc! { "_id": object_id };
        
        let registro = self.collection
            .find_one(filter)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(registro)
    }

    
}