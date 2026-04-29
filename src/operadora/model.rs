use crate::models::*;

#[derive(Serialize, Deserialize)]
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
    pub fn from(row: &Row) -> Operadora {
        Operadora {
            data_alteracao: match row.get(1) {
                Some(x) => x,
                None => "".to_string(),
            },
            responsavel: match row.get(2) {
                Some(x) => x,
                None => "".to_string(),
            },
            grupo: match row.get(3) {
                Some(x) => x,
                None => "".to_string(),
            },
            codigo_operadora: match row.get(4) {
                Some(x) => x,
                None => -1,
            },
            operadora: match row.get(5) {
                Some(x) => x,
                None => "".to_string(),
            },
            razao_social: match row.get(6) {
                Some(x) => x,
                None => "".to_string(),
            },
            cnpj: match row.get(7) {
                Some(x) => x,
                None => "".to_string(),
            },
            email: match row.get(8) {
                Some(x) => x,
                None => "".to_string(),
            },
            telefone: match row.get(9) {
                Some(x) => x,
                None => "".to_string(),
            },
        }
    }

    pub fn new(json: String) -> Result<Operadora, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
