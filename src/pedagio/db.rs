use crate::models::*;
use crate::pedagio::model::Pedagio;

pub async fn insert(pedagio: &Pedagio, pool: &Pool) -> Result<(), HttpResponse> {
    sqlx::query(
        "INSERT INTO pedagio (
            longitude, latitude, codigo_operadora, nome, situacao, rodovia, km,
            sentido, cidade, estado, codigo, orientacao, tipo, jurisdicao,
            cobranca_especial, categoria, data_alteracao, razao_social, cnpj, email, telefone
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
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

pub async fn get_all(pool: &Pool) -> Result<Vec<Pedagio>, HttpResponse> {
    sqlx::query_as::<_, Pedagio>(
        "SELECT
            longitude, latitude, codigo_operadora, nome, situacao, rodovia, km,
            sentido, cidade, estado, codigo AS codigo_pedagio, orientacao, tipo,
            jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social,
            cnpj, email, telefone
         FROM pedagio",
    )
    .fetch_all(pool)
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Erro ao buscar pedagios"))
}

pub async fn get_by_id(pool: &Pool, codigo_pedagio: i8) -> Result<Vec<Pedagio>, HttpResponse> {
    sqlx::query_as::<_, Pedagio>(
        "SELECT
            longitude, latitude, codigo_operadora, nome, situacao, rodovia, km,
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
    sqlx::query(
        "UPDATE pedagio
         SET longitude = $1, latitude = $2, codigo_operadora = $3, nome = $4, situacao = $5,
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
