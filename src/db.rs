use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use sql_builder::{quote, SqlBuilder};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
#[serde(tag = "operadora")]
struct Operadora {
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
async fn create_operadora(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let operadora: Operadora = new_operadora(data);
    let mut sql_builder = SqlBuilder::insert_into("operadora");
    sql_builder
        .field("DATA_OPERACAO")
        .field("RESPONSAVEL")
        .field("GRUPO")
        .field("CODIGO_OPERADORA")
        .field("OPERADORA")
        .field("RAZAO_SOCIAL")
        .field("CNPJ")
        .field("EMAIL")
        .field("TELEFONE");
    sql_builder.values(&[
        &quote(&operadora.data_operacao),
        &quote(&operadora.responsavel),
        &quote(&operadora.grupo),
        &quote(operadora.codigo_operadora),
        &quote(&operadora.operadora),
        &quote(&operadora.razao_social),
        &quote(&operadora.cnpj),
        &quote(&operadora.email),
        &quote(&operadora.telefone),
    ]);

    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao inserir operadora"),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(&this_sql.as_str());
    
    let result = batch_execute(&sql, pool.get_ref().clone()).await;
    match result {
        Ok(_) => return HttpResponse::Ok().body("Operadora inserida com sucesso"),
        Err(_) => return result.unwrap_err(),
    }
}

fn new_operadora(json: String) -> Operadora {
    println!("{}", json);
    let result: Operadora = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "praca")]
struct Praca {
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
async fn create_praca(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let praca: Praca = new_praca(data);
    let mut sql_builder = SqlBuilder::insert_into("praca");
    sql_builder
        .field("LONGITUDE")
        .field("LATITUDE")
        .field("ID_OPERADORA")
        .field("NOME")
        .field("SITUACAO")
        .field("RODOVIA")
        .field("KM")
        .field("SENTIDO")
        .field("CIDADE")
        .field("ESTADO")
        .field("CODIGO_PRACA")
        .field("ORIENTACAO")
        .field("TIPO")
        .field("JURISDICAO")
        .field("COBRANCA_ESPECIAL")
        .field("CATEGORIA")
        .field("DATA_DE_ALTERACAO")
        .field("RAZAO_SOCIAL")
        .field("CNPJ")
        .field("EMAIL")
        .field("TELEFONE");
    sql_builder.values(&[
        &quote(praca.longitude),
        &quote(praca.latitude),
        &quote(&praca.id_operadora),
        &quote(&praca.nome),
        &quote(&praca.situacao),
        &quote(&praca.rodovia),
        &quote(praca.km),
        &quote(&praca.sentido),
        &quote(&praca.cidade),
        &quote(&praca.estado),
        &quote(praca.codigo_praca),
        &quote(&praca.orientacao),
        &quote(&praca.tipo),
        &quote(&praca.jurisdicao),
        &quote(praca.cobranca_especial),
        &quote(&praca.categoria),
        &quote(&praca.data_de_alteracao),
        &quote(&praca.razao_social),
        &quote(&praca.cnpj),
        &quote(&praca.email),
        &quote(&praca.telefone),
    ]);
    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao inserir praça"),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(&this_sql.as_str());
    
    let result = batch_execute(&sql, pool.get_ref().clone()).await;
    match result {
        Ok(_) => return HttpResponse::Ok().body("Praca inserida com sucesso"),
        Err(_) => return result.unwrap_err(),
    }
}

fn new_praca(json: String) -> Praca {
    println!("{}", json);
    let result: Praca = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}

pub async fn batch_execute(sql: &str, pool: Pool) -> Result<(), HttpResponse> {
    let mut conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };
    let transaction = match conn.transaction().await {
        Ok(x) => x,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao iniciar transação")),
    };
    match transaction.batch_execute(sql).await {
        Ok(_) => (),
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao executar batch")),
    };
    match transaction.commit().await {
        Ok(_) => return Ok(()),
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao commitar transação")),        
    };
}