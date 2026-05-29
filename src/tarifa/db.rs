use crate::models::*;
use crate::tarifa::model::{Tarifa, TipoTarifa};
use crate::utils::ensure_situacao_enum;
use sqlx::{Postgres, QueryBuilder};

#[derive(Deserialize, Default)]
pub struct TipoTarifaFilter {
    pub q: Option<String>,
    pub id_tipo_tarifa: Option<i32>,
    pub id_padrao_tarifa: Option<i32>,
    pub descricao: Option<String>,
    pub rodagem: Option<String>,
    pub eixos: Option<i32>,
}

#[derive(Deserialize, Default)]
pub struct TarifaFilter {
    pub q: Option<String>,
    pub id_tarifa: Option<i32>,
    pub id_tipo_tarifa: Option<i32>,
    pub id_pedagio: Option<i32>,
    pub codigo_operadora: Option<i32>,
    pub situacao: Option<String>,
    pub tipo: Option<String>,
    pub descricao: Option<String>,
    pub nome: Option<String>,
    pub operadora: Option<String>,
    pub incluir_inativas: Option<bool>,
}

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

pub async fn get_all_tipos_tarifa(
    pool: &Pool,
    filter: &TipoTarifaFilter,
) -> Result<Vec<TipoTarifa>, HttpResponse> {
    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT id_tipo_tarifa, id_padrao_tarifa, descricao, tipo_rodagem, rodagem, eixos
         FROM tipo_tarifa",
    );

    let mut has_filter = false;

    if let Some(id_tipo_tarifa) = filter.id_tipo_tarifa {
        query
            .push(" WHERE id_tipo_tarifa = ")
            .push_bind(id_tipo_tarifa);
        has_filter = true;
    }
    if let Some(id_padrao_tarifa) = filter.id_padrao_tarifa {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("id_padrao_tarifa = ")
            .push_bind(id_padrao_tarifa);
        has_filter = true;
    }
    if let Some(descricao) = filter
        .descricao
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("descricao ILIKE ")
            .push_bind(format!("%{descricao}%"));
        has_filter = true;
    }
    if let Some(rodagem) = filter.rodagem.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("rodagem ILIKE ")
            .push_bind(format!("%{rodagem}%"));
        has_filter = true;
    }
    if let Some(eixos) = filter.eixos {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("eixos = ").push_bind(eixos);
        has_filter = true;
    }
    if let Some(q) = filter.q.as_deref().filter(|value| !value.is_empty()) {
        let pattern = format!("%{q}%");
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("(descricao ILIKE ").push_bind(pattern.clone());
        query.push(" OR rodagem ILIKE ").push_bind(pattern);
        query.push(")");
    }

    query
        .push(" ORDER BY id_tipo_tarifa")
        .build_query_as::<TipoTarifa>()
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
    ensure_situacao_enum(pool).await.map_err(|_| {
        HttpResponse::InternalServerError().body("Erro ao preparar enum de situação")
    })?;
    sync_tarifas_sequence(pool).await?;

    sqlx::query(
        "INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor, data_criacao,
            data_atualizacao, situacao, tipo
        ) VALUES (nextval('tarifas_id_seq'), $1, $2, $3, $4, $5, $6, $7::situacao_registro, $8)
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

pub async fn get_all_tarifas(
    pool: &Pool,
    filter: &TarifaFilter,
) -> Result<Vec<Tarifa>, HttpResponse> {
    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT
            tarifas.id_tarifa, tarifas.id_tipo_tarifa, tarifas.id_pedagio,
            tarifas.multiplicador, tarifas.valor, tarifas.data_criacao,
            tarifas.data_atualizacao, tarifas.situacao::TEXT AS situacao, tarifas.tipo,
            tipo_tarifa.descricao, tipo_tarifa.rodagem,
            COALESCE(tipo_tarifa.eixos, 0) AS eixos, pedagio.nome,
            pedagio.codigo_operadora, operadora.operadora
         FROM tarifas
         JOIN tipo_tarifa ON tarifas.id_tipo_tarifa = tipo_tarifa.id_tipo_tarifa
         JOIN pedagio ON tarifas.id_pedagio = pedagio.id_pedagio
         LEFT JOIN operadora ON pedagio.codigo_operadora = operadora.codigo_operadora",
    );

    let mut has_filter = false;

    if filter.incluir_inativas != Some(true) && filter.situacao.is_none() {
        query.push(" WHERE tarifas.situacao = 'Ativo'");
        has_filter = true;
    }
    if let Some(id_tarifa) = filter.id_tarifa {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("tarifas.id_tarifa = ").push_bind(id_tarifa);
        has_filter = true;
    }
    if let Some(id_tipo_tarifa) = filter.id_tipo_tarifa {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("tarifas.id_tipo_tarifa = ")
            .push_bind(id_tipo_tarifa);
        has_filter = true;
    }
    if let Some(id_pedagio) = filter.id_pedagio {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("tarifas.id_pedagio = ").push_bind(id_pedagio);
        has_filter = true;
    }
    if let Some(codigo_operadora) = filter.codigo_operadora {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("pedagio.codigo_operadora = ")
            .push_bind(codigo_operadora);
        has_filter = true;
    }
    if let Some(situacao) = filter.situacao.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("tarifas.situacao::TEXT ILIKE ")
            .push_bind(situacao);
        has_filter = true;
    }
    if let Some(tipo) = filter.tipo.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("tarifas.tipo ILIKE ")
            .push_bind(format!("%{tipo}%"));
        has_filter = true;
    }
    if let Some(descricao) = filter
        .descricao
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("tipo_tarifa.descricao ILIKE ")
            .push_bind(format!("%{descricao}%"));
        has_filter = true;
    }
    if let Some(nome) = filter.nome.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("pedagio.nome ILIKE ")
            .push_bind(format!("%{nome}%"));
        has_filter = true;
    }
    if let Some(operadora) = filter
        .operadora
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("operadora.operadora ILIKE ")
            .push_bind(format!("%{operadora}%"));
        has_filter = true;
    }
    if let Some(q) = filter.q.as_deref().filter(|value| !value.is_empty()) {
        let pattern = format!("%{q}%");
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("(tipo_tarifa.descricao ILIKE ")
            .push_bind(pattern.clone());
        query
            .push(" OR pedagio.nome ILIKE ")
            .push_bind(pattern.clone());
        query
            .push(" OR operadora.operadora ILIKE ")
            .push_bind(pattern.clone());
        query.push(" OR tarifas.tipo ILIKE ").push_bind(pattern);
        query.push(")");
    }

    query
        .push(" ORDER BY tarifas.id_pedagio, tarifas.id_tipo_tarifa, tarifas.id_tarifa")
        .build_query_as::<Tarifa>()
        .fetch_all(pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar tarifas"))
}

pub async fn get_tarifa_by_id(pool: &Pool, id_tarifa: i32) -> Result<Vec<Tarifa>, HttpResponse> {
    sqlx::query_as::<_, Tarifa>(
        "SELECT
            tarifas.id_tarifa, tarifas.id_tipo_tarifa, tarifas.id_pedagio,
            tarifas.multiplicador, tarifas.valor, tarifas.data_criacao,
            tarifas.data_atualizacao, tarifas.situacao::TEXT AS situacao, tarifas.tipo,
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
    ensure_situacao_enum(pool).await.map_err(|_| {
        HttpResponse::InternalServerError().body("Erro ao preparar enum de situação")
    })?;
    sync_tarifas_sequence(pool).await?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao iniciar transação"))?;

    let result = sqlx::query(
        "WITH tarifa_anterior AS (
            UPDATE tarifas
            SET situacao = 'Inativo'::situacao_registro
            WHERE id_tarifa = $6
            RETURNING id_tipo_tarifa, id_pedagio
        )
        INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor,
            data_criacao, data_atualizacao, situacao, tipo
        )
        SELECT nextval('tarifas_id_seq'), id_tipo_tarifa, id_pedagio, $1, $2, $3, $4, 'Ativo'::situacao_registro, $5
        FROM tarifa_anterior",
    )
    .bind(tarifa.multiplicador)
    .bind(tarifa.valor)
    .bind(tarifa.data_criacao)
    .bind(tarifa.data_atualizacao)
    .bind(&tarifa.tipo)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao atualizar tarifa"))?;

    if result.rows_affected() == 0 {
        return Err(HttpResponse::NotFound().body("Tarifa não encontrada"));
    }

    tx.commit()
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao commitar transação"))?;

    Ok(())
}
