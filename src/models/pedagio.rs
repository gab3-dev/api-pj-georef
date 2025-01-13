use crate::models::*;
use crate::utils::*;

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
    data_alteracao: NaiveDateTime,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

#[allow(unused)]
impl Pedagio {
    pub fn from(row: &Row) -> Self {
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

    fn new(json: String) -> Pedagio {
        let result: Pedagio = serde_json::from_str(&json.as_str()).unwrap();
        return result;
    }
}

#[post("/api/create-pedagio")]
async fn create_pedagio(data: String, pool: web::Data<Pool>) -> impl Responder {
    let mut sql = String::new();
    let pedagio: Pedagio = Pedagio::new(data);
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
            return HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str());
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

#[get("/api/get-pedagio/{codigo_pedagio}")]
async fn get_pedagio_by_id(pool: web::Data<Pool>, codigo_pedagio: web::Path<i8>) -> impl Responder {
    let mut sql = String::new();
    sql.push_str("SELECT * FROM pedagio WHERE CODIGO_PEDAGIO = ");
    sql.push_str(&codigo_pedagio.to_string());
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
            let mut pedagios = Vec::new();
            for row in rows {
                pedagios.push(Pedagio::from(&row));
            }
            return HttpResponse::Ok().json(pedagios);
        }
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao buscar pedagios"),
    }
}
