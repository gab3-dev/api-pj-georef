use crate::models::*;
use crate::tarifa::model::{Tarifa, TipoTarifa};

pub async fn insert_tipo_tarifa(tipo_tarifa: &TipoTarifa, pool: &Pool) -> Result<(), HttpResponse> {
    sqlx::query(
        "INSERT INTO tipo_tarifa (
            id_padrao_tarifa, descricao, tipo_rodagem, rodagem, eixos
        ) VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT DO NOTHING",
    )
    .bind(tipo_tarifa.id_padrao_tarifa)
    .bind(&tipo_tarifa.descricao)
    .bind(tipo_tarifa.tipo_rodagem)
    .bind(&tipo_tarifa.rodagem)
    .bind(tipo_tarifa.eixos)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao inserir tipo de tarifa"))
}

pub async fn get_all_tipos_tarifa(pool: &Pool) -> Result<Vec<TipoTarifa>, HttpResponse> {
    sqlx::query_as::<_, TipoTarifa>(
        "SELECT id_tipo_tarifa, id_padrao_tarifa, descricao, tipo_rodagem, rodagem, eixos
         FROM tipo_tarifa",
    )
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tipos de tarifa"))
}

async fn sync_tarifas_sequence(pool: &Pool) -> Result<(), HttpResponse> {
    sqlx::query("CREATE SEQUENCE IF NOT EXISTS tarifas_id_seq START WITH 1000 INCREMENT BY 1")
        .execute(pool)
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().body("Erro ao criar sequência de tarifas")
        })?;

    sqlx::query(
        "SELECT setval(
            'tarifas_id_seq',
            GREATEST(1000, (SELECT COALESCE(MAX(id_tarifa), 0) FROM tarifas))
        )",
    )
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| {
        HttpResponse::InternalServerError().body("Erro ao sincronizar sequência de tarifas")
    })
}

pub async fn insert_tarifa(tarifa: &Tarifa, pool: &Pool) -> Result<(), HttpResponse> {
    sync_tarifas_sequence(pool).await?;

    sqlx::query(
        "INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor, data_criacao,
            data_atualizacao, situacao, tipo
        ) VALUES (nextval('tarifas_id_seq'), $1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT DO NOTHING",
    )
    .bind(tarifa.id_tipo_tarifa)
    .bind(tarifa.id_pedagio)
    .bind(tarifa.multiplicador)
    .bind(tarifa.valor)
    .bind(tarifa.data_criacao)
    .bind(tarifa.data_atualizacao)
    .bind(&tarifa.situacao)
    .bind(&tarifa.tipo)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao inserir tarifa"))
}

pub async fn get_all_tarifas(pool: &Pool) -> Result<Vec<Tarifa>, HttpResponse> {
    sqlx::query_as::<_, Tarifa>(
        "SELECT
            tarifas.id_tarifa, tarifas.id_tipo_tarifa, tarifas.id_pedagio,
            tarifas.multiplicador, tarifas.valor, tarifas.data_criacao,
            tarifas.data_atualizacao, tarifas.situacao, tarifas.tipo,
            tipo_tarifa.descricao, tipo_tarifa.rodagem,
            COALESCE(tipo_tarifa.eixos, 0) AS eixos, pedagio.nome,
            pedagio.codigo_operadora, operadora.operadora
         FROM tarifas
         JOIN tipo_tarifa ON tarifas.id_tipo_tarifa = tipo_tarifa.id_tipo_tarifa
         JOIN pedagio ON tarifas.id_pedagio = pedagio.id_pedagio
         LEFT JOIN operadora ON pedagio.codigo_operadora = operadora.codigo_operadora
         WHERE tarifas.situacao = 'Ativo'",
    )
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tarifas"))
}

pub async fn get_tarifa_by_id(pool: &Pool, id_tarifa: i32) -> Result<Vec<Tarifa>, HttpResponse> {
    sqlx::query_as::<_, Tarifa>(
        "SELECT
            tarifas.id_tarifa, tarifas.id_tipo_tarifa, tarifas.id_pedagio,
            tarifas.multiplicador, tarifas.valor, tarifas.data_criacao,
            tarifas.data_atualizacao, tarifas.situacao, tarifas.tipo,
            '' AS descricao, '' AS rodagem, 0 AS eixos, '' AS nome,
            NULL::INT AS codigo_operadora, NULL::TEXT AS operadora
         FROM tarifas
         WHERE tarifas.id_tarifa = $1",
    )
    .bind(id_tarifa)
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tarifas"))
}

pub async fn update_tarifa(tarifa: &Tarifa, id: i32, pool: &Pool) -> Result<(), HttpResponse> {
    sync_tarifas_sequence(pool).await?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao iniciar transação"))?;

    sqlx::query(
        "INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor,
            data_criacao, data_atualizacao, situacao, tipo
        )
        SELECT nextval('tarifas_id_seq'), id_tipo_tarifa, id_pedagio, multiplicador, valor,
               data_criacao, data_atualizacao, 'Inativo', tipo
        FROM tarifas
        WHERE id_tarifa = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao atualizar tarifa"))?;

    sqlx::query(
        "UPDATE tarifas
         SET multiplicador = $1, valor = $2,
             data_criacao = $3, data_atualizacao = $4, situacao = $5, tipo = $6
         WHERE id_tarifa = $7",
    )
    .bind(tarifa.multiplicador)
    .bind(tarifa.valor)
    .bind(tarifa.data_criacao)
    .bind(tarifa.data_atualizacao)
    .bind(&tarifa.situacao)
    .bind(&tarifa.tipo)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao atualizar tarifa"))?;

    tx.commit()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao commitar transação"))?;

    Ok(())
}
