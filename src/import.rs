use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use actix_multipart::form::{
    tempfile::TempFile,
    MultipartForm,
};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/api/importar-operadoras")]
pub async fn import_operadoras(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    for f in form.files {
        path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
    }

    let client = pool.get().await.unwrap();

    let mut sql = String::new();
    sql.push_str(format!("COPY operadora (codigo_operadora,operadora,CNPJ,razao_social,data_alteracao,email,telefone,grupo,responsavel) FROM '{path}' DELIMITER ';' CSV HEADER ENCODING 'ISO88599';").as_str());

    let result = client.execute(&sql, &[]).await;

    match result {
        Ok(idx) => {
            log::info!("operadoras importadas com sucesso, {} rows inserted", idx);
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete operadoras.csv, it may not exist.");
            });
            Ok(HttpResponse::Ok().json(format!("{{\"operadoras_importadas\": {}}}", idx)))
        },
        Err(e) => {
            log::error!("error importing operadoras: {e}");
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete operadoras.csv, it may not exist.");
            });
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}

#[post("/api/importar-tarifas")]
pub async fn import_tarifas(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    for f in form.files {
        path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
    }

    let client = pool.get().await.unwrap();

    let mut sql = String::new();
    sql.push_str(format!("COPY tarifas FROM '{path}' DELIMITER ';' CSV HEADER;").as_str());

    let result = client.execute(&sql, &[]).await;

    match result {
        Ok(idx) => {
            log::info!("tarifas importadas com sucesso, {} rows inserted", idx);
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete tarifas.csv, it may not exist.");
            });
            Ok(HttpResponse::Ok().json(format!("{{\"tarifas_importadas\": {}}}", idx)))
        },
        Err(e) => {
            log::error!("error importing tarifas: {e}");
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete tarifas.csv, it may not exist.");
            });
            Err(actix_web::error::ErrorInternalServerError(e))
        }
    }
}
