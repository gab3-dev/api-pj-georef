use actix_web::{get, post, web, HttpResponse, Responder};
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

#[allow(unused)]
impl Operadora {    
    pub fn from(row: &Row) -> Operadora {
        Operadora {
            data_operacao: match row.get(1) {
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

#[get("/get-operadoras")]
async fn get_all_operadoras(pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM operadora;");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut operadoras = Vec::new();
            for row in rows {
                operadoras.push(Operadora::from(&row));
            }
            return HttpResponse::Ok().json(operadoras);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar operadoras"),
    }
}

#[get("/get-operadora/{codigo_operadora}")]
async fn get_operadora_by_id(pool: web::Data<Pool>, codigo_operadora: web::Path<i32>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM operadora WHERE CODIGO_OPERADORA = ");
    sql.push_str(&codigo_operadora.to_string());
    sql.push_str(";");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut operadoras = Vec::new();
            for row in rows {
                operadoras.push(Operadora::from(&row));
            }
            return HttpResponse::Ok().json(operadoras);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar operadoras"),
    }
}

fn new_operadora(json: String) -> Operadora {
    println!("{}", json);    
    let result: Operadora = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "pedagio")]
struct Pedagio {
    longitude: i64,
    latitude: i64,
    codigo_operadora: i32,
    nome: String,
    situacao: String,
    rodovia: String,
    km: i32,
    sentido: String,
    cidade: String,
    estado: String,
    codigo_pedagio: i8,
    orientacao: String,
    tipo: String,
    jurisdicao: String,
    cobranca_especial: bool,
    categoria: String,
    data_alteracao: String,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

#[allow(unused)]
impl Pedagio {    
    pub fn from(row: &Row) -> Pedagio {
        Pedagio {
            longitude: row.get(1),
            latitude: row.get(2),
            codigo_operadora: row.get(3),
            nome: row.get(4),
            situacao: row.get(5),
            rodovia: row.get(6),
            km: row.get(7),
            sentido: row.get(8),
            cidade: row.get(9),
            estado: row.get(10),
            codigo_pedagio: row.get(11),
            orientacao: row.get(12),
            tipo: row.get(13),
            jurisdicao: row.get(14),
            cobranca_especial: row.get(15),
            categoria: row.get(16),
            data_alteracao: row.get(17),
            razao_social: row.get(18),
            cnpj: row.get(19),
            email: row.get(20),
            telefone: row.get(21),
        }        
    }
}

#[post("/create-pedagio")]
async fn create_pedagio(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let pedagio: Pedagio = new_pedagio(data);
    let mut sql_builder = SqlBuilder::insert_into("pedagio");
    sql_builder
        .field("LONGITUDE")
        .field("LATITUDE")
        .field("CODIGO_OPERADORA")
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
        .field("DATA_ALTERACAO")
        .field("RAZAO_SOCIAL")
        .field("CNPJ")
        .field("EMAIL")
        .field("TELEFONE");
    sql_builder.values(&[
        &quote(pedagio.longitude),
        &quote(pedagio.latitude),
        &quote(&pedagio.codigo_operadora),
        &quote(&pedagio.nome),
        &quote(&pedagio.situacao),
        &quote(&pedagio.rodovia),
        &quote(pedagio.km),
        &quote(&pedagio.sentido),
        &quote(&pedagio.cidade),
        &quote(&pedagio.estado),
        &quote(pedagio.codigo_pedagio),
        &quote(&pedagio.orientacao),
        &quote(&pedagio.tipo),
        &quote(&pedagio.jurisdicao),
        &quote(pedagio.cobranca_especial),
        &quote(&pedagio.categoria),
        &quote(&pedagio.data_alteracao),
        &quote(&pedagio.razao_social),
        &quote(&pedagio.cnpj),
        &quote(&pedagio.email),
        &quote(&pedagio.telefone),
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
        Ok(_) => return HttpResponse::Ok().body("Pedagio inserida com sucesso"),
        Err(_) => return result.unwrap_err(),
    }
}

#[get("/get-pedagios")]
async fn get_all_pedagio(pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM pedagio;");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut pedagios = Vec::new();
            for row in rows {
                pedagios.push(Operadora::from(&row));
            }
            return HttpResponse::Ok().json(pedagios);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar pedagios"),
    }
}

#[get("/get-pedagio/{codigo_pedagio}")]
async fn get_pedagio_by_id(pool: web::Data<Pool>, codigo_pedagio: web::Path<i8>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM pedagio WHERE CODIGO_PEDAGIO = ");
    sql.push_str(&codigo_pedagio.to_string());
    sql.push_str(";");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut pedagios = Vec::new();
            for row in rows {
                pedagios.push(Pedagio::from(&row));
            }
            return HttpResponse::Ok().json(pedagios);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar pedagios"),
    }
}

fn new_pedagio(json: String) -> Pedagio {
    println!("{}", json);
    let result: Pedagio = serde_json::from_str(&json.as_str()).unwrap();

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