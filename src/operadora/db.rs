use crate::models::*;
use crate::operadora::model::Operadora;
use sqlx::{Postgres, QueryBuilder};

#[derive(Deserialize, Default)]
pub struct OperadoraFilter {
    pub q: Option<String>,
    pub codigo_operadora: Option<i32>,
    pub operadora: Option<String>,
    pub grupo: Option<String>,
    pub cnpj: Option<String>,
    pub email: Option<String>,
}

pub async fn insert(operadora: &Operadora, pool: &Pool) -> Result<(), HttpResponse> {
    sqlx::query(
        "INSERT INTO operadora (
            data_alteracao, responsavel, grupo, codigo_operadora, operadora,
            razao_social, cnpj, email, telefone
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT DO NOTHING",
    )
    .bind(&operadora.data_alteracao)
    .bind(&operadora.responsavel)
    .bind(&operadora.grupo)
    .bind(operadora.codigo_operadora)
    .bind(&operadora.operadora)
    .bind(&operadora.razao_social)
    .bind(&operadora.cnpj)
    .bind(&operadora.email)
    .bind(&operadora.telefone)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao inserir operadora"))
}

pub async fn get_all(
    pool: &Pool,
    filter: &OperadoraFilter,
) -> Result<Vec<Operadora>, HttpResponse> {
    let mut query = QueryBuilder::<Postgres>::new(
        "SELECT
            data_alteracao, responsavel, grupo, codigo_operadora, operadora,
            razao_social, cnpj, email, telefone
         FROM operadora",
    );

    let mut has_filter = false;

    if let Some(codigo_operadora) = filter.codigo_operadora {
        query
            .push(" WHERE codigo_operadora = ")
            .push_bind(codigo_operadora);
        has_filter = true;
    }
    if let Some(operadora) = filter
        .operadora
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query
            .push("operadora ILIKE ")
            .push_bind(format!("%{operadora}%"));
        has_filter = true;
    }
    if let Some(grupo) = filter.grupo.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("grupo ILIKE ").push_bind(format!("%{grupo}%"));
        has_filter = true;
    }
    if let Some(cnpj) = filter.cnpj.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("cnpj ILIKE ").push_bind(format!("%{cnpj}%"));
        has_filter = true;
    }
    if let Some(email) = filter.email.as_deref().filter(|value| !value.is_empty()) {
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("email ILIKE ").push_bind(format!("%{email}%"));
        has_filter = true;
    }
    if let Some(q) = filter.q.as_deref().filter(|value| !value.is_empty()) {
        let pattern = format!("%{q}%");
        query.push(if has_filter { " AND " } else { " WHERE " });
        query.push("(operadora ILIKE ").push_bind(pattern.clone());
        query
            .push(" OR razao_social ILIKE ")
            .push_bind(pattern.clone());
        query.push(" OR cnpj ILIKE ").push_bind(pattern.clone());
        query.push(" OR email ILIKE ").push_bind(pattern);
        query.push(")");
    }

    query
        .push(" ORDER BY codigo_operadora")
        .build_query_as::<Operadora>()
        .fetch_all(pool)
        .await
        .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar operadoras"))
}

pub async fn get_by_id(pool: &Pool, codigo_operadora: i32) -> Result<Vec<Operadora>, HttpResponse> {
    sqlx::query_as::<_, Operadora>(
        "SELECT
            data_alteracao, responsavel, grupo, codigo_operadora, operadora,
            razao_social, cnpj, email, telefone
         FROM operadora
         WHERE codigo_operadora = $1",
    )
    .bind(codigo_operadora)
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar operadoras"))
}

pub async fn update(
    operadora: &Operadora,
    codigo_operadora: i32,
    pool: &Pool,
) -> Result<(), HttpResponse> {
    sqlx::query(
        "UPDATE operadora
         SET data_alteracao = $1, responsavel = $2, grupo = $3, operadora = $4,
             razao_social = $5, cnpj = $6, email = $7, telefone = $8
         WHERE codigo_operadora = $9",
    )
    .bind(&operadora.data_alteracao)
    .bind(&operadora.responsavel)
    .bind(&operadora.grupo)
    .bind(&operadora.operadora)
    .bind(&operadora.razao_social)
    .bind(&operadora.cnpj)
    .bind(&operadora.email)
    .bind(&operadora.telefone)
    .bind(codigo_operadora)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao atualizar operadora"))
}
