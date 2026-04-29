use crate::models::*;

#[derive(Serialize, Deserialize)]
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
    pub fn from(row: &Row) -> TipoTarifa {
        TipoTarifa {
            id_tipo_tarifa: row.get(0),
            id_padrao_tarifa: row.get(1),
            descricao: row.get(2),
            tipo_rodagem: row.get(3),
            rodagem: row.get(4),
            eixos: row.get(5),
        }
    }

    pub fn new(json: String) -> Result<TipoTarifa, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Serialize, Deserialize)]
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
}

#[allow(unused)]
impl Tarifa {
    pub fn from(row: &Row) -> Tarifa {
        Tarifa {
            id_tarifa: row.get("id_tarifa"),
            id_tipo_tarifa: row.get("id_tipo_tarifa"),
            id_pedagio: row.get("id_pedagio"),
            multiplicador: row.get("multiplicador"),
            valor: row.get("valor"),
            data_criacao: row.get("data_criacao"),
            data_atualizacao: row.get("data_atualizacao"),
            situacao: row.get("situacao"),
            tipo: row.get("tipo"),
            descricao: row.get("descricao"),
            rodagem: row.get("rodagem"),
            eixos: row.get("eixos"),
            nome: row.get("nome"),
        }
    }

    pub fn new(json: String) -> Result<Tarifa, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
