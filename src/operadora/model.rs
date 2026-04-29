use crate::models::*;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Operadora {
    pub data_alteracao: String,
    pub responsavel: String,
    pub grupo: String,
    pub codigo_operadora: i32,
    pub operadora: String,
    pub razao_social: String,
    pub cnpj: String,
    pub email: String,
    pub telefone: String,
}

#[allow(unused)]
impl Operadora {
    pub fn new(json: String) -> Result<Operadora, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
