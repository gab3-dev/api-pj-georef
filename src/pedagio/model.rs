use crate::models::*;

#[derive(Serialize, Deserialize)]
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
    pub fn from(row: &Row) -> Self {
        Pedagio {
            longitude: row.get("longitude"),
            latitude: row.get("latitude"),
            codigo_operadora: row.get("codigo_operadora"),
            nome: row.get("nome"),
            situacao: row.get("situacao"),
            rodovia: row.get("rodovia"),
            km: row.get("km"),
            sentido: row.get("sentido"),
            cidade: row.get("cidade"),
            estado: row.get("estado"),
            codigo_pedagio: row.get("codigo"),
            orientacao: row.get("orientacao"),
            tipo: row.get("tipo"),
            jurisdicao: row.get("jurisdicao"),
            cobranca_especial: row.get("cobranca_especial"),
            categoria: row.get("categoria"),
            data_alteracao: row.get("data_alteracao"),
            razao_social: row.get("razao_social"),
            cnpj: row.get("cnpj"),
            email: row.get("email"),
            telefone: row.get("telefone"),
        }
    }

    pub fn new(json: String) -> Result<Pedagio, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
