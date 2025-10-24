use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registro {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub estado: Estado,
    pub regional: Regional,
    pub responsavel: String,
    pub data: NaiveDate,
    pub payload: String,
}

impl Registro {
    pub const fn new(
        estado: Estado,
        regional: Regional,
        responsavel: String,
        data: NaiveDate,
        payload: String,
    ) -> Self {
        Self {
            id: None,
            estado,
            regional,
            responsavel,
            data,
            payload,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
    TO,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
    Uo1,
}

impl Estado {
    pub const fn obter_regional(self) -> Regional {
        match self {
            Self::AC => Regional::Gr11Uo2,
            Self::AL => Regional::Gr06Uo1,
            Self::AP => Regional::Gr11,
            Self::AM => Regional::Gr10Uo2,
            Self::BA => Regional::Gr08,
            Self::CE => Regional::Gr09,
            Self::DF => Regional::Uo1,
            Self::ES => Regional::Gr02Uo1,
            Self::GO => Regional::Gr07,
            Self::MA => Regional::Gr10Uo1,
            Self::MT => Regional::Gr07Uo1,
            Self::MS => Regional::Gr07Uo2,
            Self::MG => Regional::Gr04,
            Self::PA => Regional::Gr10,
            Self::PB => Regional::Gr06Uo2,
            Self::PR => Regional::Gr03,
            Self::PE => Regional::Gr06,
            Self::PI => Regional::Gr09Uo2,
            Self::RJ => Regional::Gr02,
            Self::RN => Regional::Gr09Uo1,
            Self::RS => Regional::Gr05,
            Self::RO => Regional::Gr11Uo1,
            Self::RR => Regional::Gr11Uo3,
            Self::SC => Regional::Gr03Uo1,
            Self::SP => Regional::Gr01,
            Self::SE => Regional::Gr08Uo1,
            Self::TO => Regional::Gr07Uo3,
        }
    }
}