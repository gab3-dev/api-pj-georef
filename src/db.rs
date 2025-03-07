use actix_web::{get, post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use sql_builder::{quote, SqlBuilder};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize)]
#[serde(tag = "operadora")]
struct Operadora {
    data_alteracao: String,
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
}

#[post("/api/create-operadora")]
async fn create_operadora(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let operadora: Operadora = new_operadora(data);
    let mut sql_builder = SqlBuilder::insert_into("operadora");
    sql_builder
        .field("DATA_ALTERACAO")
        .field("RESPONSAVEL")
        .field("GRUPO")
        .field("CODIGO_OPERADORA")
        .field("OPERADORA")
        .field("RAZAO_SOCIAL")
        .field("CNPJ")
        .field("EMAIL")
        .field("TELEFONE");
    sql_builder.values(&[
        &quote(&operadora.data_alteracao),
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

#[get("/api/get-operadoras")]
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

#[get("/api/get-operadora/{codigo_operadora}")]
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

#[post("/api/create-pedagio")]
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

#[get("/api/get-pedagios")]
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

#[get("/api/get-pedagio/{codigo_pedagio}")]
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "tipo_tarifa")]
struct TipoTarifa {
    id_tipo_tarifa: i32,
    id_padrao_tarifa: Option<i32>,
    descricao: String,
    tipo_rodagem: Option<i32>,
    rodagem: String,
    eixos: Option<i32>,
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

    pub fn new_tipo_tarifa(json: String) -> TipoTarifa {
        let result: TipoTarifa = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-tipo-tarifa")]
async fn create_tipo_tarifa(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let tipo_tarifa: TipoTarifa = TipoTarifa::new_tipo_tarifa(data);
    let mut sql_builder = SqlBuilder::insert_into("tipo_tarifa");
    sql_builder
        .field("ID_PADRAO_TARIFA")
        .field("DESCRICAO")
        .field("TIPO_RODAGEM")
        .field("RODAGEM")
        .field("EIXOS");
    sql_builder.values(&[
        &quote(tipo_tarifa.id_padrao_tarifa.map(|x| x.to_string()).unwrap_or("NULL".to_string())),
        &quote(tipo_tarifa.descricao),
        &quote(tipo_tarifa.tipo_rodagem.map(|x| x.to_string()).unwrap_or("NULL".to_string())),
        &quote(tipo_tarifa.rodagem),
        &quote(tipo_tarifa.eixos.map(|x| x.to_string()).unwrap_or("NULL".to_string())),
    ]);
    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao inserir tipo de tarifa"),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(&this_sql.as_str());
    
    let result = batch_execute(&sql, pool.get_ref().clone()).await;
    match result {
        Ok(_) => return HttpResponse::Ok().body("Tipo de tarifa inserida com sucesso"),
        Err(_) => return result.unwrap_err(),
    }
}

#[get("/api/get-tipos-tarifa")]
async fn get_all_tipos_tarifa(pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM tipo_tarifa;");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut tipos_tarifa = Vec::new();
            for row in rows {
                tipos_tarifa.push(TipoTarifa::from(&row));
            }
            return HttpResponse::Ok().json(tipos_tarifa);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar tipos de tarifa"),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "tarifa")]
struct Tarifa {
    id_tarifa: i32,
    id_tipo_tarifa: i32,
    id_pedagio: i32,
    multiplicador: f64,
    valor: f64,
    data_criacao: String,
    data_atualizacao: String,
    situacao: String,
    tipo: String,
}

#[allow(unused)]
impl Tarifa {
    pub fn from(row: &Row) -> Tarifa {
        Tarifa {
            id_tarifa: row.get(0),
            id_tipo_tarifa: row.get(1),
            id_pedagio: row.get(2),
            multiplicador: row.get(3),
            valor: row.get(4),
            data_criacao: row.get(5),
            data_atualizacao: row.get(6),
            situacao: row.get(7),
            tipo: row.get(8),
        }
    }

    pub fn new_tarifa(json: String) -> Tarifa {
        let result: Tarifa = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-tarifa")]
async fn create_tarifa(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let tarifa: Tarifa = Tarifa::new_tarifa(data);
    let mut sql_builder = SqlBuilder::insert_into("tarifa");
    sql_builder
        .field("ID_TIPO_TARIFA")
        .field("ID_PEDAGIO")
        .field("MULTIPLICADOR")
        .field("VALOR")
        .field("DATA_CRIACAO")
        .field("DATA_ATUALIZACAO")
        .field("SITUACAO")
        .field("TIPO");
    sql_builder.values(&[
        &quote(tarifa.id_tipo_tarifa),
        &quote(tarifa.id_pedagio),
        &quote(tarifa.multiplicador),
        &quote(tarifa.valor),
        &quote(tarifa.data_criacao),
        &quote(tarifa.data_atualizacao),
        &quote(tarifa.situacao),
        &quote(tarifa.tipo),
    ]);
    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao inserir tarifa"),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(&this_sql.as_str());
    
    let result = batch_execute(&sql, pool.get_ref().clone()).await;
    match result {
        Ok(_) => return HttpResponse::Ok().body("Tarifa inserida com sucesso"),
        Err(_) => return result.unwrap_err(),
    }
}

#[get("/api/get-tarifas")]
async fn get_all_tarifas(pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM tarifa;");
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    match result {
        Ok(rows) => {
            let mut tarifas = Vec::new();
            for row in rows {
                tarifas.push(Tarifa::from(&row));
            }
            return HttpResponse::Ok().json(tarifas);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar tarifas"),
    }
}

#[get("/api/get-tarifa/{id_tarifa}")]
async fn get_tarifa_by_id(pool: web::Data<Pool>, id_tarifa: web::Path<i32>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM tarifa WHERE ID_TARIFA = ");
    sql.push_str(&id_tarifa.to_string());
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
            let mut tarifas = Vec::new();
            for row in rows {
                tarifas.push(Tarifa::from(&row));
            }
            return HttpResponse::Ok().json(tarifas);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar tarifas"),
    }
}