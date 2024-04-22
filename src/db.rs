use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
#[serde(tag = "operadora")]
struct Operadora {
    id: String,
    data_operacao: String,
    responsavel: String,
    grupo: String,
    codigo_operadora: i32,
    operadora: String,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

impl Operadora {
    pub fn from(row: &Row) -> Operadora {
        Operadora {
            id: row.get(0),
            data_operacao: row.get(1),
            responsavel: row.get(2),
            grupo: row.get(3),
            codigo_operadora: row.get(4),
            operadora: row.get(5),
            razao_social: row.get(6),
            cnpj: row.get(7),
            email: row.get(8),
            telefone: row.get(9),
        }
    }
}

#[post("/create-operadora")]
async fn create_operadora(data: String) -> impl Responder {
    let operadora = serde_json::to_string(&new_operadora(data)).unwrap();
    HttpResponse::Ok().body(operadora)
}

fn new_operadora(json: String) -> Operadora {
    println!("{}", json);
    let result: Operadora = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "praca")]
struct Praca {
    id: String,
    longitude: i32,
    latitude: i32,
    id_operadora: String,
    nome: String,
    situacao: String,
    rodovia: String,
    km: i32,
    sentido: String,
    cidade: String,
    estado: String,
    codigo_praca: i8,
    orientacao: String,
    tipo: String,
    jurisdicao: String,
    cobranca_especial: bool,
    categoria: String,
    data_de_alteracao: String,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

impl Praca {
    pub fn from(row: &Row) -> Praca {
        Praca {
            id: row.get(0),
            longitude: row.get(1),
            latitude: row.get(2),
            id_operadora: row.get(3),
            nome: row.get(4),
            situacao: row.get(5),
            rodovia: row.get(6),
            km: row.get(7),
            sentido: row.get(8),
            cidade: row.get(9),
            estado: row.get(10),
            codigo_praca: row.get(11),
            orientacao: row.get(12),
            tipo: row.get(13),
            jurisdicao: row.get(14),
            cobranca_especial: row.get(15),
            categoria: row.get(16),
            data_de_alteracao: row.get(17),
            razao_social: row.get(18),
            cnpj: row.get(19),
            email: row.get(20),
            telefone: row.get(21),
        }        
    }
}

#[post("/create-praca")]
async fn create_praca(data: String) -> impl Responder {
    let praca = serde_json::to_string(&new_praca(data)).unwrap();
    HttpResponse::Ok().body(praca)
}

fn new_praca(json: String) -> Praca {
    println!("{}", json);
    let result: Praca = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}