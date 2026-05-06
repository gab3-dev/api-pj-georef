use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fs, fs::File, fs::Permissions, os::unix::fs::PermissionsExt};

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
    Ok(columns
        .iter()
        .map(|c| c.trim())
        .collect::<Vec<&str>>()
        .join(","))
}

fn safe_file_name(file_name: Option<String>, fallback: &str) -> String {
    file_name
        .as_deref()
        .and_then(|name| Path::new(name).file_name())
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(fallback)
        .to_string()
}

fn uploaded_paths(file_name: &str) -> (String, String) {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./tmp".to_string());
    let db_upload_dir = std::env::var("DB_UPLOAD_DIR").unwrap_or_else(|_| "/uploaded".to_string());
    let app_path = Path::new(&upload_dir).join(file_name);
    let db_path = Path::new(&db_upload_dir).join(file_name);

    (
        app_path.to_string_lossy().to_string(),
        db_path.to_string_lossy().replace('\'', "''"),
    )
}

fn remove_uploaded_file(path: &str, label: &str) {
    if path.is_empty() {
        return;
    }

    fs::remove_file(path).unwrap_or_else(|e| {
        log::warn!("Failed to delete uploaded {label} CSV at {path}: {e}");
    });
}

#[post("/api/importar-operadoras")]
pub async fn import_operadoras(
    _admin: AdminAutenticado,
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<PgPool>,
) -> Result<impl Responder, actix_web::Error> {
    let mut path = String::new();
    let mut path_on_server = String::new();
    for f in form.files {
        let file_name = safe_file_name(f.file_name, "operadoras.csv");
        (path, path_on_server) = uploaded_paths(&file_name);
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664)).unwrap();
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
    let mut path_on_server = String::new();
    for f in form.files {
        let file_name = safe_file_name(f.file_name, "tarifas.csv");
        (path, path_on_server) = uploaded_paths(&file_name);
        log::info!("saving to {path}");
        f.file.persist(path.clone()).unwrap();
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664)).unwrap();
    }

    if path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "erro": "Nenhum arquivo CSV foi enviado."
        })));
    }

    // Read header dynamically
    let columns = match read_csv_header(&path) {
        Ok(cols) => cols,
        Err(e) => {
            log::error!("Failed to read CSV header: {}", e);
            remove_uploaded_file(&path, "tarifas");
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "erro": "Falha ao ler o cabeçalho do arquivo CSV."
            })));
        }
    };

    let column_names = columns.split(',').map(str::trim).collect::<Vec<&str>>();

    if !column_names.iter().any(|column| *column == "id_tarifa") {
        remove_uploaded_file(&path, "tarifas");
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "erro": "O arquivo CSV de tarifas deve conter a coluna id_tarifa."
        })));
    }

    log::info!("importing tarifas from {path_on_server}");
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("error acquiring database connection for tarifas import: {e}");
            remove_uploaded_file(&path, "tarifas");
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "erro": "Falha ao importar o arquivo de tarifas.",
                "detalhes": e.to_string()
            })));
        }
    };

    let reset_temp_table = "DROP TABLE IF EXISTS import_tarifas_tmp";
    let result = async {
        sqlx::query(reset_temp_table).execute(&mut *conn).await?;
        sqlx::query("CREATE TEMP TABLE import_tarifas_tmp (LIKE tarifas INCLUDING DEFAULTS)")
            .execute(&mut *conn)
            .await?;

        let copy_sql = format!(
            "COPY import_tarifas_tmp ({columns}) \
             FROM '{path_on_server}' \
             WITH (FORMAT csv, HEADER true, DELIMITER ';', ENCODING 'UTF8');"
        );
        sqlx::query(&copy_sql).execute(&mut *conn).await?;

        let duplicate_id = sqlx::query_scalar::<_, i32>(
            "SELECT id_tarifa \
             FROM import_tarifas_tmp \
             GROUP BY id_tarifa \
             HAVING COUNT(*) > 1 \
             LIMIT 1",
        )
        .fetch_optional(&mut *conn)
        .await?;

        if let Some(id_tarifa) = duplicate_id {
            return Ok(Err(id_tarifa));
        }

        let tarifas_atualizadas = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) \
             FROM import_tarifas_tmp tmp \
             WHERE EXISTS ( \
                 SELECT 1 FROM tarifas t WHERE t.id_tarifa = tmp.id_tarifa \
             )",
        )
        .fetch_one(&mut *conn)
        .await?;

        let tarifas_importadas =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM import_tarifas_tmp")
                .fetch_one(&mut *conn)
                .await?;

        let update_assignments = column_names
            .iter()
            .filter(|column| **column != "id_tarifa")
            .map(|column| format!("{column} = EXCLUDED.{column}"))
            .collect::<Vec<String>>()
            .join(", ");
        let conflict_action = if update_assignments.is_empty() {
            "DO NOTHING".to_string()
        } else {
            format!("DO UPDATE SET {update_assignments}")
        };

        let upsert_sql = format!(
            "INSERT INTO tarifas ({columns}) \
             SELECT {columns} FROM import_tarifas_tmp \
             ON CONFLICT (id_tarifa) {conflict_action}"
        );
        sqlx::query(&upsert_sql).execute(&mut *conn).await?;
        sqlx::query(reset_temp_table).execute(&mut *conn).await?;

        Ok::<Result<(i64, i64), i32>, sqlx::Error>(Ok((
            tarifas_importadas - tarifas_atualizadas,
            tarifas_atualizadas,
        )))
    }
    .await;

    match result {
        Ok(Ok((tarifas_inseridas, tarifas_atualizadas))) => {
            log::info!(
                "tarifas importadas com sucesso, {} inserted and {} updated",
                tarifas_inseridas,
                tarifas_atualizadas
            );
            remove_uploaded_file(&path, "tarifas");
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "tarifas_inseridas": tarifas_inseridas,
                "tarifas_atualizadas": tarifas_atualizadas,
                "tarifas_importadas": tarifas_inseridas + tarifas_atualizadas
            })))
        }
        Ok(Err(id_tarifa)) => {
            let _ = sqlx::query(reset_temp_table).execute(&mut *conn).await;
            remove_uploaded_file(&path, "tarifas");
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "erro": "O arquivo CSV contém id_tarifa duplicado.",
                "detalhes": format!("O id_tarifa {id_tarifa} aparece mais de uma vez no arquivo.")
            })))
        }
        Err(e) => {
            let _ = sqlx::query(reset_temp_table).execute(&mut *conn).await;
            log::error!("error importing tarifas: {e}");
            remove_uploaded_file(&path, "tarifas");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "erro": "Falha ao importar o arquivo de tarifas.",
                "detalhes": e.to_string()
            })))
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
    let mut path_on_server = String::new();
    // 1. Salva o arquivo enviado em um diretório temporário
    for f in form.files {
        // Usa um nome de arquivo fixo ou o nome original, como preferir
        let file_name = safe_file_name(f.file_name, "pedagios.csv");
        (path, path_on_server) = uploaded_paths(&file_name);
        log::info!("Salvando arquivo de pedágios em: {}", path);
        f.file.persist(path.clone()).unwrap();

        // Define permissões para que o processo do Postgres possa ler o arquivo
        let file = File::open(path.clone()).unwrap();
        file.set_permissions(Permissions::from_mode(0o664)).unwrap();
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
