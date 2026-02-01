use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::{Pool, Client};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use std::{fs, fs::Permissions, fs::File, os::unix::fs::PermissionsExt};

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
        file.set_permissions(Permissions::from_mode(0o664))
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
        file.set_permissions(Permissions::from_mode(0o664))
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

#[post("/api/importar-pedagios")]
pub async fn import_pedagios(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<Pool>,
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

    // 2. Define o caminho do arquivo como o container do Postgres o enxerga
    //    Isso assume um volume compartilhado: ./tmp do seu app -> /uploaded do Postgres
    let path_on_server = path.clone().replace("./tmp/", "/uploaded/");

    log::info!("Iniciando importação de pedágios de: {}", path_on_server);
    let client: Client = pool
        .get()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // 3. Monta o comando SQL COPY
    //    A lista de colunas deve corresponder exatamente à ordem no seu arquivo CSV
    //    CSV Header: id_pedagio;Longitude;Latitude;Nome;codigo_operadora;Concessionaria;Situacao;Sigla;rodovia;Km;id_trecho;Sentido;Cidade;Estado;Codigo;orientacao;Tipo;juridicao;cobranca_especial;Categoria;data_alteracao;razao_social;CNPJ;Email;tefefone
    let sql = format!(
        "COPY pedagio (id_pedagio, longitude, latitude, nome, codigo_operadora, concessionaria, situacao, sigla, rodovia, km, id_trecho, sentido, cidade, estado, codigo, orientacao, tipo, jurisdicao, cobranca_especial, categoria, data_alteracao, razao_social, cnpj, email, telefone) \
         FROM '{path_on_server}' \
         DELIMITER ';' \
         CSV HEADER \
         ENCODING 'ISO88599';" // Mantém encoding ISO88599 para compatibilidade com operadora
    );

    log::info!("Executando SQL: {}", sql);
    let result = client.execute(sql.as_str(), &[]).await;

    // 4. Trata o resultado da importação
    match result {
        Ok(rows_affected) => {
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
