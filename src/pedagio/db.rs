use crate::models::*;
use crate::pedagio::model::Pedagio;
use crate::utils::ensure_situacao_enum;
use sqlx::{Postgres, QueryBuilder};

#[derive(Deserialize, Default)]
pub struct PedagioFilter {
    pub q: Option<String>,
    pub id_pedagio: Option<i32>,
    pub codigo_operadora: Option<i32>,
    pub nome: Option<String>,
    pub situacao: Option<String>,
    pub rodovia: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub codigo: Option<String>,
}

pub async fn insert(pedagio: &Pedagio, pool: &Pool) -> Result<(), HttpResponse> {
    ensure_situacao_enum(pool).await.map_err(|_| {
        HttpResponse::InternalServerError().body("Erro ao preparar enum de situação")
    })?;

    sqlx::query(
        "INSERT INTO pedagio (
            longitude, latitude, codigo_operadora, nome, situacao, rodovia, km,
            sentido, cidade, estado, codigo, orientacao, tipo, jurisdicao,
            cobranca_especial, categoria, data_alteracao, razao_social, cnpj, email, telefone
        ) VALUES (
            $1, $2, $3, $4, $5::situacao_registro, $6, $7, $8, $9, $10, $11, $12, $13, $14,
            $15, $16, $17, $18, $19, $20, $21
        )
        ON CONFLICT DO NOTHING",
    )
    .bind(pedagio.longitude)
    .bind(pedagio.latitude)
    .bind(pedagio.codigo_operadora)
    .bind(&pedagio.nome)
    .bind(&pedagio.situacao)
    .bind(&pedagio.rodovia)
    .bind(pedagio.km)
    .bind(&pedagio.sentido)
    .bind(&pedagio.cidade)
    .bind(&pedagio.estado)
    .bind(&pedagio.codigo_pedagio)
    .bind(&pedagio.orientacao)
    .bind(&pedagio.tipo)
    .bind(&pedagio.jurisdicao)
    .bind(pedagio.cobranca_especial)
    .bind(&pedagio.categoria)
    .bind(&pedagio.data_alteracao)
    .bind(&pedagio.razao_social)
    .bind(&pedagio.cnpj)
    .bind(&pedagio.email)
    .bind(&pedagio.telefone)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao inserir praça"))
}

pub async fn get_all(pool: &Pool, filter: &PedagioFilter) -> Result<Vec<Pedagio>, HttpResponse> {
    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT
            id_pedagio, longitude, latitude, codigo_operadora, nome, situacao::TEXT AS situacao, rodovia, km,
            sentido, cidade, estado, codigo AS codigo_pedagio, orientacao, tipo,
            jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social,
            cnpj, email, telefone
         FROM pedagio",
    );

    let mut has_filter = false;

    if let Some(id_pedagio) = filter.id_pedagio {
        query.push(" WHERE id_pedagio = ").push_bind(id_pedagio);
        has_filter = true;
    }
    if let Some(codigo_operadora) = filter.codigo_operadora {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("codigo_operadora = ")
            .push_bind(codigo_operadora);
        has_filter = true;
    }
    if let Some(nome) = filter.nome.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("nome ILIKE ").push_bind(format!("%{nome}%"));
        has_filter = true;
    }
    if let Some(situacao) = filter.situacao.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("situacao::TEXT ILIKE ")
            .push_bind(format!("%{situacao}%"));
        has_filter = true;
    }
    if let Some(rodovia) = filter.rodovia.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("rodovia ILIKE ")
            .push_bind(format!("%{rodovia}%"));
        has_filter = true;
    }
    if let Some(cidade) = filter.cidade.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("cidade ILIKE ").push_bind(format!("%{cidade}%"));
        has_filter = true;
    }
    if let Some(estado) = filter.estado.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("estado ILIKE ").push_bind(estado);
        has_filter = true;
    }
    if let Some(codigo) = filter.codigo.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("codigo ILIKE ").push_bind(format!("%{codigo}%"));
        has_filter = true;
    }
    if let Some(q) = filter.q.as_deref().filter(|value| !value.is_empty()) {
        let pattern = format!("%{q}%");
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("(nome ILIKE ").push_bind(pattern.clone());
        query.push(" OR rodovia ILIKE ").push_bind(pattern.clone());
        query.push(" OR cidade ILIKE ").push_bind(pattern.clone());
        query.push(" OR estado ILIKE ").push_bind(pattern.clone());
        query.push(" OR codigo ILIKE ").push_bind(pattern);
        query.push(")");
    }

    query
        .push(" ORDER BY id_pedagio")
        .build_query_as::<Pedagio>()
        .fetch_all(pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar pedagios"))
}

pub async fn get_by_id(pool: &Pool, codigo_pedagio: i8) -> Result<Vec<Pedagio>, HttpResponse> {
    sqlx::query_as::<_, Pedagio>(
        "SELECT
            id_pedagio, longitude, latitude, codigo_operadora, nome, situacao::TEXT AS situacao, rodovia, km,
            sentido, cidade, estado, codigo AS codigo_pedagio, orientacao, tipo,
            jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social,
            cnpj, email, telefone
         FROM pedagio
         WHERE codigo_pedagio = $1",
    )
    .bind(codigo_pedagio)
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar pedagios"))
}

pub async fn update(
    pedagio: &Pedagio,
    codigo_pedagio: String,
    pool: &Pool,
) -> Result<(), HttpResponse> {
    ensure_situacao_enum(pool).await.map_err(|_| {
        HttpResponse::InternalServerError().body("Erro ao preparar enum de situação")
    })?;

    sqlx::query(
        "UPDATE pedagio
         SET longitude = $1, latitude = $2, codigo_operadora = $3, nome = $4, situacao = $5::situacao_registro,
             rodovia = $6, km = $7, sentido = $8, cidade = $9, estado = $10, codigo = $11,
             orientacao = $12, tipo = $13, jurisdicao = $14, cobranca_especial = $15,
             categoria = $16, data_alteracao = $17, razao_social = $18, cnpj = $19,
             email = $20, telefone = $21
         WHERE codigo = $22",
    )
    .bind(pedagio.longitude)
    .bind(pedagio.latitude)
    .bind(pedagio.codigo_operadora)
    .bind(&pedagio.nome)
    .bind(&pedagio.situacao)
    .bind(&pedagio.rodovia)
    .bind(pedagio.km)
    .bind(&pedagio.sentido)
    .bind(&pedagio.cidade)
    .bind(&pedagio.estado)
    .bind(&pedagio.codigo_pedagio)
    .bind(&pedagio.orientacao)
    .bind(&pedagio.tipo)
    .bind(&pedagio.jurisdicao)
    .bind(pedagio.cobranca_especial)
    .bind(&pedagio.categoria)
    .bind(&pedagio.data_alteracao)
    .bind(&pedagio.razao_social)
    .bind(&pedagio.cnpj)
    .bind(&pedagio.email)
    .bind(&pedagio.telefone)
    .bind(codigo_pedagio)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao atualizar pedagio"))
}
