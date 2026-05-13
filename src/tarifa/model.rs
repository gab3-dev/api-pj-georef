use crate::models::*;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(tag = "tipo_tarifa")]
pub struct TipoTarifa {
    pub id_tipo_tarifa: i32,
    pub id_padrao_tarifa: Option<i32>,
    pub descricao: String,
    pub tipo_rodagem: Option<i32>,
    pub rodagem: String,
    pub eixos: Option<i32>,
}

#[allow(unused)]
impl TipoTarifa {
    pub fn new(json: String) -> Result<TipoTarifa, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Tarifa {
    pub id_tarifa: i32,
    pub id_tipo_tarifa: i32,
    pub id_pedagio: i32,
    pub multiplicador: f64,
    pub valor: f64,
    pub data_criacao: NaiveDateTime,
    pub data_atualizacao: NaiveDateTime,
    pub situacao: String,
    pub tipo: String,
    pub descricao: String,
    pub rodagem: String,
    pub eixos: i32,
    pub nome: String,
    #[serde(default)]
    pub codigo_operadora: Option<i32>,
    #[serde(default)]
    pub operadora: Option<String>,
}

#[allow(unused)]
impl Tarifa {
    pub fn new(json: String) -> Result<Tarifa, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
