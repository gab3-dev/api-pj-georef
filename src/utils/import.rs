use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use std::{fs::File, os::unix::fs::PermissionsExt};

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
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(std::fs::Permissions::from_mode(0o664))
            .unwrap();
    }

    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("importing operadoras from {path_on_server}");
    let client = pool.get().await.unwrap();

    let mut sql = String::new();
    sql.push_str(format!("COPY operadora (codigo_operadora,operadora,CNPJ,razao_social,data_alteracao,email,telefone,grupo,responsavel) FROM '{path_on_server}' DELIMITER ';' CSV HEADER ENCODING 'ISO88599';").as_str());

    log::info!("executing {sql}");
    let result = client.execute(&sql, &[]).await;
    log::info!("executed");

    match result {
        Ok(idx) => {
            log::info!("operadoras importadas com sucesso, all rows inserted");
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("{{\"operadoras_importadas\": {}}}", idx)))
        }
        Err(e) => {
            if e.to_string().contains(
                "duplicate key value violates unique constraint \"operadora_codigo_operadora_key\"",
            ) {
                let error_msg = e.to_string();
                if let Some(detail_start) = error_msg.find("DETAIL:") {
                    let details = error_msg[detail_start..]
                        .replace("DETAIL: Key (codigo_operadora)=(", "Código da operadora ")
                        .replace(") already exists.", " já existe.");

                    log::warn!("operadoras já importadas, removendo arquivo");
                    std::fs::remove_file(path).unwrap_or_else(|_| {
                        log::warn!("Failed to delete operadoras.csv, it may not exist.");
                    });
                    return Ok(HttpResponse::Ok()
                    .json(format!("{{\"operadoras_importadas\": 0, \"erro\": \"operadoras já importadas\", \"detalhes\": \"{}\"}}", details)));
                } else {
                    log::error!("error importing operadoras: {e}");
                    std::fs::remove_file(path).unwrap_or_else(|_| {
                        log::warn!("Failed to delete operadoras.csv, it may not exist.");
                    });
                    return Err(actix_web::error::ErrorInternalServerError(e));
                }
            }
            log::error!("error importing operadoras: {e}");
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete operadoras.csv, it may not exist.");
            });
            return Ok(HttpResponse::InternalServerError().json(format!("{{\"erro\": \"{}\"}}", e)));
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
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(std::fs::Permissions::from_mode(0o664))
            .unwrap();
    }

    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("importing tarifas from {path_on_server}");
    let client = pool.get().await.unwrap();

    let mut sql = String::new();
    sql.push_str(
        format!("COPY tarifas FROM '{path_on_server}' DELIMITER ';' CSV HEADER;").as_str(),
    );

    let result = client.execute(&sql, &[]).await;

    match result {
        Ok(idx) => {
            log::info!("tarifas importadas com sucesso, {} rows inserted", idx);
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete tarifas.csv, it may not exist.");
            });
            Ok(HttpResponse::Ok().json(format!("{{\"tarifas_importadas\": {}}}", idx)))
        }
        Err(e) => {
            log::error!("error importing tarifas: {e}");
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete tarifas.csv, it may not exist.");
            });
            return Ok(HttpResponse::InternalServerError().json(format!("{{\"erro\": \"{}\"}}", e)));
        }
    }
}
