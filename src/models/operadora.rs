use crate::models::*;
use crate::utils::*;

#[derive(Serialize, Deserialize)]
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
    fn from(row: &Row) -> Operadora {
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

    fn new(json: String) -> Operadora {
        let result: Operadora = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-operadora")]
async fn create_operadora(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let operadora: Operadora = Operadora::new(data);
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
            return HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
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
async fn get_operadora_by_id(
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM operadora WHERE CODIGO_OPERADORA = ");
    sql.push_str(&codigo_operadora.to_string());
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
            let mut operadoras = Vec::new();
            for row in rows {
                operadoras.push(Operadora::from(&row));
            }
            return HttpResponse::Ok().json(operadoras);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar operadoras"),
    }
}
