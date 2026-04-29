use crate::models::*;
use crate::operadora::model::Operadora;
use crate::utils::batch_execute;

pub fn build_insert_sql(operadora: &Operadora) -> Result<String, HttpResponse> {
    let mut sql = String::new();
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
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao inserir operadora")),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(this_sql.as_str());
    Ok(sql)
}

pub async fn insert(operadora: &Operadora, pool: &Pool) -> Result<(), HttpResponse> {
    let sql = build_insert_sql(operadora)?;
    batch_execute(&sql, pool.clone()).await
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    conn.query("SELECT * FROM operadora;", &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar operadoras"))
}

pub async fn get_by_id(pool: &Pool, codigo_operadora: i32) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    let sql = format!(
        "SELECT * FROM operadora WHERE CODIGO_OPERADORA = {};",
        codigo_operadora
    );

    conn.query(sql.as_str(), &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar operadoras"))
}

pub fn build_update_sql(
    operadora: &Operadora,
    codigo_operadora: i32,
) -> Result<String, HttpResponse> {
    let mut sql_builder = SqlBuilder::update_table("operadora");
    sql_builder
        .set("DATA_ALTERACAO", &quote(&operadora.data_alteracao))
        .set("RESPONSAVEL", &quote(&operadora.responsavel))
        .set("GRUPO", &quote(&operadora.grupo))
        .set("OPERADORA", &quote(&operadora.operadora))
        .set("RAZAO_SOCIAL", &quote(&operadora.razao_social))
        .set("CNPJ", &quote(&operadora.cnpj))
        .set("EMAIL", &quote(&operadora.email))
        .set("TELEFONE", &quote(&operadora.telefone));
    sql_builder.and_where_eq("CODIGO_OPERADORA", codigo_operadora);

    match sql_builder.sql() {
        Ok(x) => Ok(x),
        Err(_) => Err(HttpResponse::InternalServerError().body("Erro ao atualizar operadora")),
    }
}

pub async fn update(
    operadora: &Operadora,
    codigo_operadora: i32,
    pool: &Pool,
) -> Result<(), HttpResponse> {
    let sql = build_update_sql(operadora, codigo_operadora)?;
    batch_execute(&sql, pool.clone()).await
}
