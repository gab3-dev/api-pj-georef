use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use std::{fs, fs::Permissions, fs::File, os::unix::fs::PermissionsExt};
use std::io::{BufRead, BufReader};

use crate::auth::AdminAutenticado;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

fn read_csv_header(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    // Strip UTF-8 BOM if present
    let header = line.trim().trim_start_matches('\u{feff}');
    if header.is_empty() {
        return Err("Empty CSV file".into());
    }
    // Split by semicolon, validate each column name
    let columns: Vec<&str> = header.split(';').collect();
    for col in &columns {
        let col = col.trim();
        if col.is_empty() {
            return Err("Nome de coluna vazio no cabeçalho do CSV".into());
        }
        if !col.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(format!(
                "Nome de coluna inválido no cabeçalho do CSV: '{}'. Apenas letras, números e underscore são permitidos.",
                col
            ).into());
        }
    }
    Ok(columns.iter().map(|c| c.trim()).collect::<Vec<&str>>().join(","))
}

#[post("/api/importar-operadoras")]
pub async fn import_operadoras(
    _admin: AdminAutenticado,
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    for f in form.files {
        path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664))
            .unwrap();
    }

    // Read header dynamically
    let columns = match read_csv_header(&path) {
        Ok(cols) => cols,
        Err(e) => {
            log::error!("Failed to read CSV header: {}", e);
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "erro": "Falha ao ler o cabeçalho do arquivo CSV."
            })));
        }
    };

    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("importing operadoras from {path_on_server}");
    let mut sql = String::new();
    sql.push_str(format!("COPY operadora ({columns}) FROM '{path_on_server}' WITH (FORMAT csv, HEADER true, DELIMITER ';', ENCODING 'UTF8');").as_str());

    log::info!("executing {sql}");
    let result = sqlx::query(&sql).execute(pool.get_ref()).await;
    log::info!("executed");

    match result {
        Ok(result) => {
            let rows_affected = result.rows_affected();
            log::info!("operadoras importadas com sucesso, all rows inserted");
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(format!("{{\"operadoras_importadas\": {}}}", rows_affected)))
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
    _admin: AdminAutenticado,
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    for f in form.files {
        path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664))
            .unwrap();
    }

    // Read header dynamically
    let columns = match read_csv_header(&path) {
        Ok(cols) => cols,
        Err(e) => {
            log::error!("Failed to read CSV header: {}", e);
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "erro": "Falha ao ler o cabeçalho do arquivo CSV."
            })));
        }
    };

    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("importing tarifas from {path_on_server}");
    let mut sql = String::new();
    sql.push_str(
        format!("COPY tarifas ({columns}) FROM '{path_on_server}' WITH (FORMAT csv, HEADER true, DELIMITER ';', ENCODING 'UTF8');").as_str(),
    );

    let result = sqlx::query(&sql).execute(pool.get_ref()).await;

    match result {
        Ok(result) => {
            let rows_affected = result.rows_affected();
            log::info!("tarifas importadas com sucesso, {} rows inserted", rows_affected);
            std::fs::remove_file(path).unwrap_or_else(|_| {
                log::warn!("Failed to delete tarifas.csv, it may not exist.");
            });
            Ok(HttpResponse::Ok().json(format!("{{\"tarifas_importadas\": {}}}", rows_affected)))
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

#[post("/api/importar-pedagios")]
pub async fn import_pedagios(
    _admin: AdminAutenticado,
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    // 1. Salva o arquivo enviado em um diretório temporário
    for f in form.files {
        // Usa um nome de arquivo fixo ou o nome original, como preferir
        let file_name = f.file_name.unwrap_or_else(|| "pedagios.csv".to_string());
        path = format!("./tmp/{}", file_name);
        log::info!("Salvando arquivo de pedágios em: {}", path);
        f.file.persist(path.clone()).unwrap();

        // Define permissões para que o processo do Postgres possa ler o arquivo
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664))
            .unwrap();
    }

    // Read header dynamically
    let columns = match read_csv_header(&path) {
        Ok(cols) => cols,
        Err(e) => {
            log::error!("Failed to read CSV header: {}", e);
             return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "erro": "Falha ao ler o cabeçalho do arquivo CSV."
            })));
        }
    };

    // 2. Define o caminho do arquivo como o container do Postgres o enxerga
    //    Isso assume um volume compartilhado: ./tmp do seu app -> /uploaded do Postgres
    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("Iniciando importação de pedágios de: {}", path_on_server);
    // 3. Monta o comando SQL COPY
    //    Usamos as colunas lidas dinamicamente do CSV
    let sql = format!(
        "COPY pedagio ({columns}) \
         FROM '{path_on_server}' \
         WITH (FORMAT csv, HEADER true, DELIMITER ';', ENCODING 'UTF8');"
    );

    log::info!("Executando SQL: {}", sql);
    let result = sqlx::query(sql.as_str()).execute(pool.get_ref()).await;

    // 4. Trata o resultado da importação
    match result {
        Ok(result) => {
            let rows_affected = result.rows_affected();
            log::info!("{} pedágios importados com sucesso.", rows_affected);
            // Limpa o arquivo temporário após o sucesso
            fs::remove_file(path)
                .unwrap_or_else(|e| log::warn!("Falha ao deletar arquivo temporário: {}", e));
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "pedagios_importados": rows_affected
            })))
        }
        Err(e) => {
            log::error!("Erro ao importar pedágios: {}", e);
            // Sempre tenta limpar o arquivo temporário, mesmo em caso de erro
            fs::remove_file(path.clone())
                .unwrap_or_else(|err| log::warn!("Falha ao deletar arquivo temporário: {}", err));

            // Tratamento de erro específico para chave duplicada (ex: codigo do pedágio já existe)
            // IMPORTANTE: Ajuste o nome da constraint "pedagio_codigo_key" para o nome real da sua tabela.
            if e.to_string()
                .contains("violates unique constraint \"pedagio_codigo_key\"")
            {
                let error_msg = e.to_string();
                let details = if let Some(detail_start) = error_msg.find("DETAIL:") {
                    error_msg[detail_start..]
                        .replace("DETAIL: Key (codigo)=(", "O pedágio com código ")
                        .replace(") already exists.", " já existe.")
                } else {
                    "Um dos códigos de pedágio no arquivo já existe no banco de dados.".to_string()
                };

                return Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "erro": "Pedágio duplicado.",
                    "detalhes": details
                })));
            }

            // Erro genérico
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "erro": "Falha ao importar o arquivo de pedágios.",
                "detalhes": e.to_string()
            })))
        }
    }
}
