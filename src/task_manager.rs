use crate::models::{Registro, Estado, Regional};
use crate::error::AppError;
use chrono::Local;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TaskManager {
    pub estado: Estado,
    pub responsavel: String,
    pub payload: String,
}

#[derive(Deserialize)]
struct Config {
    caminho_drive: String,
}

impl TaskManager {
    pub fn processar(self) -> Result<String, AppError> {
        let regional = self.estado.obter_regional();
        
        let registro = Registro::new(
            self.estado,
            regional,
            self.responsavel,
            Local::now().naive_local().date(),
            self.payload,
        );
        
        let json_string = serde_json::to_string_pretty(&registro)?;
        let caminho = Self::gerar_caminho(&registro)?;
        
        if let Some(dir) = Path::new(&caminho).parent() {
            fs::create_dir_all(dir)?;
        }
        
        fs::write(&caminho, json_string)?;
        
        Ok(caminho)
    }
    
    fn gerar_caminho(registro: &Registro) -> Result<String, AppError> {
        // Lê a configuração
        let config_str = fs::read_to_string("config.json")
            .map_err(|_| AppError::InvalidData(
                "Configuração não encontrada. Configure o caminho do Drive primeiro.".into()
            ))?;
        
        let config: Config = serde_json::from_str(&config_str)
            .map_err(|_| AppError::InvalidData("Configuração inválida".into()))?;
        
        let estado_str = format!("{:?}", registro.estado);
        let regional_str = serde_json::to_string(&registro.regional)?
            .trim_matches('"')
            .to_string();
        
        let caminho = format!(
            r"{}\{}\{}\{}.json",
            config.caminho_drive,
            estado_str,
            regional_str,
            registro.data
        );
        
        Ok(caminho)
    }
}