use crate::models::*;
use crate::pedagio::model::Pedagio;
use crate::utils::batch_execute;

pub fn build_insert_sql(pedagio: &Pedagio) -> Result<String, HttpResponse> {
    let mut sql = String::new();
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
        .field("CODIGO")
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
        &quote(&pedagio.codigo_pedagio),
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
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao inserir praça")),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(this_sql.as_str());
    Ok(sql)
}

pub async fn insert(pedagio: &Pedagio, pool: &Pool) -> Result<(), HttpResponse> {
    let sql = build_insert_sql(pedagio)?;
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

    let sql = "SELECT longitude, latitude, codigo_operadora, nome, situacao, rodovia, km, sentido, cidade, estado, \
        codigo, orientacao, tipo, jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social, \
        cnpj, email, telefone FROM pedagio;";

    conn.query(sql, &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar pedagios"))
}

pub async fn get_by_id(pool: &Pool, codigo_pedagio: i8) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    let sql = format!(
        "SELECT * FROM pedagio WHERE CODIGO_PEDAGIO = {};",
        codigo_pedagio
    );

    conn.query(sql.as_str(), &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar pedagios"))
}

pub fn build_update_sql(pedagio: &Pedagio, codigo_pedagio: String) -> Result<String, HttpResponse> {
    let mut sql_builder = SqlBuilder::update_table("pedagio");
    sql_builder
        .set("LONGITUDE", &quote(pedagio.longitude))
        .set("LATITUDE", &quote(pedagio.latitude))
        .set("CODIGO_OPERADORA", &quote(pedagio.codigo_operadora))
        .set("NOME", &quote(&pedagio.nome))
        .set("SITUACAO", &quote(&pedagio.situacao))
        .set("RODOVIA", &quote(&pedagio.rodovia))
        .set("KM", &quote(pedagio.km))
        .set("SENTIDO", &quote(&pedagio.sentido))
        .set("CIDADE", &quote(&pedagio.cidade))
        .set("ESTADO", &quote(&pedagio.estado))
        .set("CODIGO", &quote(&pedagio.codigo_pedagio))
        .set("ORIENTACAO", &quote(&pedagio.orientacao))
        .set("TIPO", &quote(&pedagio.tipo))
        .set("JURISDICAO", &quote(&pedagio.jurisdicao))
        .set("COBRANCA_ESPECIAL", &quote(pedagio.cobranca_especial))
        .set("CATEGORIA", &quote(&pedagio.categoria))
        .set("DATA_ALTERACAO", &quote(&pedagio.data_alteracao))
        .set("RAZAO_SOCIAL", &quote(&pedagio.razao_social))
        .set("CNPJ", &quote(&pedagio.cnpj))
        .set("EMAIL", &quote(&pedagio.email))
        .set("TELEFONE", &quote(&pedagio.telefone));
    sql_builder.and_where_eq("CODIGO", &quote(codigo_pedagio));

    match sql_builder.sql() {
        Ok(x) => Ok(x),
        Err(_) => Err(HttpResponse::InternalServerError().body("Erro ao atualizar pedagio")),
    }
}

pub async fn update(
    pedagio: &Pedagio,
    codigo_pedagio: String,
    pool: &Pool,
) -> Result<(), HttpResponse> {
    let sql = build_update_sql(pedagio, codigo_pedagio)?;
    batch_execute(&sql, pool.clone()).await
}
