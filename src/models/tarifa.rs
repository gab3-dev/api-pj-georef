use crate::models::*;
use crate::utils::*;

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
    fn from(row: &Row) -> TipoTarifa {
        TipoTarifa {
            id_tipo_tarifa: row.get(0),
            id_padrao_tarifa: row.get(1),
            descricao: row.get(2),
            tipo_rodagem: row.get(3),
            rodagem: row.get(4),
            eixos: row.get(5),
        }
    }

    fn new(json: String) -> TipoTarifa {
        let result: TipoTarifa = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-tipo-tarifa")]
async fn create_tipo_tarifa(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let tipo_tarifa: TipoTarifa = TipoTarifa::new(data);
    let mut sql_builder = SqlBuilder::insert_into("tipo_tarifa");
    sql_builder
        .field("ID_PADRAO_TARIFA")
        .field("DESCRICAO")
        .field("TIPO_RODAGEM")
        .field("RODAGEM")
        .field("EIXOS");
    sql_builder.values(&[
        &quote(
            tipo_tarifa
                .id_padrao_tarifa
                .map(|x| x.to_string())
                .unwrap_or("NULL".to_string()),
        ),
        &quote(tipo_tarifa.descricao),
        &quote(
            tipo_tarifa
                .tipo_rodagem
                .map(|x| x.to_string())
                .unwrap_or("NULL".to_string()),
        ),
        &quote(tipo_tarifa.rodagem),
        &quote(
            tipo_tarifa
                .eixos
                .map(|x| x.to_string())
                .unwrap_or("NULL".to_string()),
        ),
    ]);
    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Erro ao inserir tipo de tarifa")
        }
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
            return HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
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
        Err(_) => {
            return HttpResponse::InternalServerError().body("Erro ao buscar tipos de tarifa")
        }
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
    data_criacao: NaiveDateTime,
    data_atualizacao: NaiveDateTime,
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

    fn new(json: String) -> Tarifa {
        let result: Tarifa = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-tarifa")]
async fn create_tarifa(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let tarifa: Tarifa = Tarifa::new(data);
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
    log::info!("sql: {}", sql);
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
        }
    };
    let result = conn.query(sql.as_str(), &[]).await;
    log::info!("result: {:?}", result);
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
            return HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
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
