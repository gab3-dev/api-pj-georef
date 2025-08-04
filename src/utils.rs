pub use actix_web::HttpResponse;
pub use deadpool_postgres::Pool;

pub mod files;
pub mod import;
pub use files::*;
pub use import::*;

pub async fn batch_execute(sql: &str, pool: Pool) -> Result<(), HttpResponse> {
    let mut conn = match pool.get().await {
        Ok(x) => x,
        Err(e) => {
            return Err(HttpResponse::InternalServerError()
                .body("Erro ao conectar ao banco de dados".to_owned() + e.to_string().as_str()));
        }
    };
    let transaction = match conn.transaction().await {
        Ok(x) => x,
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao iniciar transação")),
    };
    match transaction.batch_execute(sql).await {
        Ok(_) => (),
        Err(_) => return Err(HttpResponse::InternalServerError().body("Erro ao executar batch")),
    };
    match transaction.commit().await {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(HttpResponse::InternalServerError().body("Erro ao commitar transação"))
        }
    };
}
