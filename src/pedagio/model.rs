use crate::models::*;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(tag = "pedagio")]
pub struct Pedagio {
    pub longitude: i64,
    pub latitude: i64,
    pub codigo_operadora: i32,
    pub nome: String,
    pub situacao: String,
    pub rodovia: String,
    pub km: f64,
    pub sentido: String,
    pub cidade: String,
    pub estado: String,
    pub codigo_pedagio: String,
    pub orientacao: String,
    pub tipo: String,
    pub jurisdicao: String,
    pub cobranca_especial: bool,
    pub categoria: String,
    pub data_alteracao: String,
    pub razao_social: String,
    pub cnpj: String,
    pub email: String,
    pub telefone: String,
}

impl Pedagio {
    pub fn new(json: String) -> Result<Pedagio, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
