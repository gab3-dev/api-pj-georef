use crate::models::*;
use crate::tarifa::model::{Tarifa, TipoTarifa};
use crate::utils::batch_execute;

pub fn build_tipo_tarifa_insert_sql(tipo_tarifa: &TipoTarifa) -> Result<String, HttpResponse> {
    let mut sql = String::new();
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
        &quote(&tipo_tarifa.descricao),
        &quote(
            tipo_tarifa
                .tipo_rodagem
                .map(|x| x.to_string())
                .unwrap_or("NULL".to_string()),
        ),
        &quote(&tipo_tarifa.rodagem),
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
            return Err(HttpResponse::InternalServerError().body("Erro ao inserir tipo de tarifa"));
        }
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(this_sql.as_str());
    Ok(sql)
}

pub async fn insert_tipo_tarifa(tipo_tarifa: &TipoTarifa, pool: &Pool) -> Result<(), HttpResponse> {
    let sql = build_tipo_tarifa_insert_sql(tipo_tarifa)?;
    batch_execute(&sql, pool.clone()).await
}

pub async fn get_all_tipos_tarifa(pool: &Pool) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    conn.query("SELECT * FROM tipo_tarifa;", &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tipos de tarifa"))
}

pub fn build_tarifa_insert_sql(tarifa: &Tarifa) -> Result<String, HttpResponse> {
    let mut sql = String::new();
    let mut sql_builder = SqlBuilder::insert_into("tarifas");
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
        &quote(&tarifa.situacao),
        &quote(&tarifa.tipo),
    ]);
    let mut this_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao inserir tarifa")),
    };
    this_sql.pop();
    this_sql.push_str("ON CONFLICT DO NOTHING;");
    sql.push_str(this_sql.as_str());
    Ok(sql)
}

pub async fn insert_tarifa(tarifa: &Tarifa, pool: &Pool) -> Result<(), HttpResponse> {
    let sql = build_tarifa_insert_sql(tarifa)?;
    batch_execute(&sql, pool.clone()).await
}

pub async fn get_all_tarifas(pool: &Pool) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    let mut sql = String::new();
    sql.push_str(
        "SELECT id_tarifa, pedagio.nome, descricao, multiplicador, valor, rodagem, eixos, ",
    );
    sql.push_str("data_criacao, data_atualizacao, tarifas.situacao, tarifas.tipo, ");
    sql.push_str("tipo_tarifa.id_tipo_tarifa, id_padrao_tarifa, tarifas.id_pedagio ");
    sql.push_str("FROM tarifas ");
    sql.push_str("JOIN tipo_tarifa ON tarifas.id_tipo_tarifa = tipo_tarifa.id_tipo_tarifa ");
    sql.push_str("JOIN pedagio ON tarifas.id_pedagio = pedagio.id_pedagio ");
    sql.push_str("WHERE tarifas.situacao = 'Ativo';");
    log::info!("sql: {}", sql);

    conn.query(sql.as_str(), &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tarifas"))
}

pub async fn get_tarifa_by_id(pool: &Pool, id_tarifa: i32) -> Result<Vec<Row>, HttpResponse> {
    let conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };

    let sql = format!("SELECT * FROM tarifas WHERE ID_TARIFA = {};", id_tarifa);

    conn.query(sql.as_str(), &[])
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tarifas"))
}

pub fn build_tarifa_update_sql(tarifa: &Tarifa, id: i32) -> Result<String, HttpResponse> {
    let backup_sql = format!(
        "INSERT INTO tarifas (id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor, data_criacao, data_atualizacao, situacao, tipo) \
         SELECT nextval('tarifas_id_seq'), id_tipo_tarifa, id_pedagio, multiplicador, valor, data_criacao, data_atualizacao, 'Inativo', tipo \
         FROM tarifas WHERE id_tarifa = {};",
        id
    );

    let mut sql_builder = SqlBuilder::update_table("tarifas");
    sql_builder
        .set("ID_TIPO_TARIFA", &quote(tarifa.id_tipo_tarifa))
        .set("ID_PEDAGIO", &quote(tarifa.id_pedagio))
        .set("MULTIPLICADOR", &quote(tarifa.multiplicador))
        .set("VALOR", &quote(tarifa.valor))
        .set("DATA_CRIACAO", &quote(tarifa.data_criacao))
        .set("DATA_ATUALIZACAO", &quote(tarifa.data_atualizacao))
        .set("SITUACAO", &quote(&tarifa.situacao))
        .set("TIPO", &quote(&tarifa.tipo));
    sql_builder.and_where_eq("ID_TARIFA", id);

    let update_sql = match sql_builder.sql() {
        Ok(x) => x,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao atualizar tarifa")),
    };

    Ok(format!("{}{}", backup_sql, update_sql))
}

pub async fn update_tarifa(tarifa: &Tarifa, id: i32, pool: &Pool) -> Result<(), HttpResponse> {
    let sql = build_tarifa_update_sql(tarifa, id)?;
    batch_execute(&sql, pool.clone()).await
}
