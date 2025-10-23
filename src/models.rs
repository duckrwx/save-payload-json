use chrono::NaiveDate;
use serde::{ Deserialize, Serialize };
#[derive(Debug, Serialize, Deserialize)]
pub struct Registro {
    pub estado: Estado,
    pub regional: Regional,
    pub responsavel: String,
    pub data: NaiveDate,
    pub payload: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Estado {
    #[serde(rename = "Acre")]
    AC,
    #[serde(rename = "Alagoas")]
    AL,
    #[serde(rename = "Amapá")]
    AP,
    #[serde(rename = "Amazonas")]
    AM,
    #[serde(rename = "Bahia")]
    BA,
    #[serde(rename = "Ceará")]
    CE,
    #[serde(rename = "Distrito Federal")]
    DF,
    #[serde(rename = "Espírito Santo")]
    ES,
    #[serde(rename = "Goiás")]
    GO,
    #[serde(rename = "Maranhão")]
    MA,
    #[serde(rename = "Mato Grosso")]
    MT,
    #[serde(rename = "Mato Grosso do Sul")]
    MS,
    #[serde(rename = "Minas Gerais")]
    MG,
    #[serde(rename = "Pará")]
    PA,
    #[serde(rename = "Paraíba")]
    PB,
    #[serde(rename = "Paraná")]
    PR,
    #[serde(rename = "Pernambuco")]
    PE,
    #[serde(rename = "Piauí")]
    PI,
    #[serde(rename = "Rio de Janeiro")]
    RJ,
    #[serde(rename = "Rio Grande do Norte")]
    RN,
    #[serde(rename = "Rio Grande do Sul")]
    RS,
    #[serde(rename = "Rondônia")]
    RO,
    #[serde(rename = "Roraima")]
    RR,
    #[serde(rename = "Santa Catarina")]
    SC,
    #[serde(rename = "São Paulo")]
    SP,
    #[serde(rename = "Sergipe")]
    SE,
    #[serde(rename = "Tocantins")]
    TO
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Regional {
    #[serde(rename = "GR01")]
    Gr01,
    #[serde(rename = "GR02")]
    Gr02,
    #[serde(rename = "GR02 UO02.1")]
    Gr02Uo1,
    #[serde(rename = "GR03")]
    Gr03,
    #[serde(rename = "GR03 UO03.1")]
    Gr03Uo1,
    #[serde(rename = "GR04")]
    Gr04,
    #[serde(rename = "GR05")]
    Gr05,
    #[serde(rename = "GR06")]
    Gr06,
    #[serde(rename = "GR06 UO06.1")]
    Gr06Uo1,
    #[serde(rename = "GR06 UO06.2")]
    Gr06Uo2,
    #[serde(rename = "GR07")]
    Gr07,
    #[serde(rename = "GR07 UO07.1")]
    Gr07Uo1,
    #[serde(rename = "GR07 UO07.2")]
    Gr07Uo2,
    #[serde(rename = "GR07 UO07.3")]
    Gr07Uo3,
    #[serde(rename = "GR08")]
    Gr08,
    #[serde(rename = "GR08 UO08.1")]
    Gr08Uo1,
    #[serde(rename = "GR09")]
    Gr09,
    #[serde(rename = "GR09 UO09.1")]
    Gr09Uo1,
    #[serde(rename = "GR09 UO09.2")]
    Gr09Uo2,
    #[serde(rename = "GR010")]
    Gr10,
    #[serde(rename = "GR010 UO010.1")]
    Gr10Uo1,
    #[serde(rename = "GR010 UO010.2")]
    Gr10Uo2,
    #[serde(rename = "GR011")]
    Gr11,
    #[serde(rename = "GR011 UO011.1")]
    Gr11Uo1,
    #[serde(rename = "GR011 UO011.2")]
    Gr11Uo2,
    #[serde(rename = "GR011 UO011.3")]
    Gr11Uo3,
    #[serde(rename = "UO001")]
    Uo1
}

impl Registro {
    pub fn new(
        estado: Estado,
        regional: Regional,
        responsavel: String,
        data: NaiveDate,
        payload: String
    ) -> Self {
        Self {
            estado,
            regional,
            responsavel,
            data,
            payload
        }
    }
}
impl Estado {
    /// Retorna a Regional correspondente ao Estado
    pub fn obter_regional(&self) -> Regional {
        match self {
            Estado::AC => Regional::Gr11Uo2,
            Estado::AL => Regional::Gr06Uo1,
            Estado::AP => Regional::Gr11,
            Estado::AM => Regional::Gr10Uo2, 
            Estado::BA => Regional::Gr08,
            Estado::CE => Regional::Gr09,
            Estado::DF => Regional::Uo1,
            Estado::ES => Regional::Gr02Uo1,
            Estado::GO => Regional::Gr07,
            Estado::MA => Regional::Gr10Uo1,
            Estado::MT => Regional::Gr07Uo1,
            Estado::MS => Regional::Gr07Uo2,
            Estado::MG => Regional::Gr04,
            Estado::PA => Regional::Gr10,
            Estado::PB => Regional::Gr06Uo2,
            Estado::PR => Regional::Gr03,
            Estado::PE => Regional::Gr06,
            Estado::PI => Regional::Gr09Uo2,
            Estado::RJ => Regional::Gr02,
            Estado::RN => Regional::Gr09Uo1,
            Estado::RS => Regional::Gr05,
            Estado::RO => Regional::Gr11Uo1,
            Estado::RR => Regional::Gr11Uo3,
            Estado::SC => Regional::Gr03Uo1,
            Estado::SP => Regional::Gr01,
            Estado::SE => Regional::Gr08Uo1,
            Estado::TO => Regional::Gr07Uo3,
        }
    }
}