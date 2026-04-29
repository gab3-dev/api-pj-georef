use crate::models::*;
use crate::operadora::model::Operadora;

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

pub async fn get_all(pool: &Pool) -> Result<Vec<Operadora>, HttpResponse> {
    sqlx::query_as::<_, Operadora>(
        "SELECT
            data_alteracao, responsavel, grupo, codigo_operadora, operadora,
            razao_social, cnpj, email, telefone
         FROM operadora",
    )
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
