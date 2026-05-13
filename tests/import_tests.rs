//! Tests for Import endpoints
//!
//! Note: These tests require multipart form handling which is more complex to test.
//! The tests here focus on verifying the endpoints are reachable and handle errors gracefully.

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{http::StatusCode, test, web, App};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;

use bgm::auth::models::Claims;
use bgm::auth::JwtConfig;
use bgm::utils::*;

const TEST_SECRET: &str = "test_secret_key";

async fn create_test_pool() -> sqlx::PgPool {
    let db_host = std::env::var("DB_HOST").unwrap_or("localhost".into());
    let db_port = std::env::var("DB_PORT").unwrap_or("5432".into());
    let db_name = std::env::var("DB_NAME").unwrap_or("pj_georef".into());
    let db_user = std::env::var("DB_USER").unwrap_or("root".into());
    let db_password = std::env::var("DB_PASSWORD").unwrap_or("1234".into());
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}

fn jwt_config() -> JwtConfig {
    JwtConfig {
        secret: TEST_SECRET.to_string(),
        expiration_seconds: 8 * 3600,
    }
}

fn admin_token() -> String {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: "admin@test.com".to_string(),
        perfil: "admin".to_string(),
        nome: "Admin".to_string(),
        iat: now,
        exp: now + 3600,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
    )
    .unwrap()
}

fn configure_upload_paths() -> String {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./tmp".to_string());
    std::fs::create_dir_all(&upload_dir).unwrap();
    std::env::set_var("UPLOAD_DIR", &upload_dir);

    if std::env::var("DB_UPLOAD_DIR").is_err() {
        std::env::set_var("DB_UPLOAD_DIR", "/uploaded");
    }

    upload_dir
}

fn multipart_csv(filename: &str, csv: &str) -> (String, Vec<u8>) {
    let boundary = format!("boundary-{filename}");
    let body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n\
         Content-Type: text/csv\r\n\r\n\
         {csv}\r\n\
         --{boundary}--\r\n"
    )
    .into_bytes();

    (boundary, body)
}

async fn post_tarifas_csv(pool: sqlx::PgPool, filename: &str, csv: &str) -> (StatusCode, Value) {
    let upload_dir = configure_upload_paths();
    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory(upload_dir))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_tarifas_rest),
    )
    .await;

    let (boundary, body) = multipart_csv(filename, csv);
    let req = test::TestRequest::post()
        .uri("/api/imports/tarifas")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .insert_header((
            "Content-Type",
            format!("multipart/form-data; boundary={boundary}"),
        ))
        .set_payload(body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    let json = serde_json::from_slice(&body)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&body).to_string()));

    (status, json)
}

async fn reset_tarifas(pool: &sqlx::PgPool, ids: &[i32]) {
    sqlx::query("DELETE FROM tarifas WHERE id_tarifa = ANY($1)")
        .bind(ids)
        .execute(pool)
        .await
        .unwrap();
}

async fn seed_tarifa(pool: &sqlx::PgPool, id_tarifa: i32, valor: f64) {
    sqlx::query(
        "INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor,
            data_criacao, data_atualizacao, situacao, tipo
        )
        VALUES ($1, 1, 1, 1.0, $2, '2024-01-01', '2024-01-01', 'Ativo', 'Normal')
        ON CONFLICT (id_tarifa) DO UPDATE
        SET valor = EXCLUDED.valor",
    )
    .bind(id_tarifa)
    .bind(valor)
    .execute(pool)
    .await
    .unwrap();
}

async fn tarifa_valor(pool: &sqlx::PgPool, id_tarifa: i32) -> f64 {
    sqlx::query_scalar::<_, f64>("SELECT valor FROM tarifas WHERE id_tarifa = $1")
        .bind(id_tarifa)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[actix_rt::test]
async fn test_import_operadoras_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_operadoras_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/operadoras")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_import_tarifas_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_tarifas_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/tarifas")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_import_tarifas_csv_missing_id_tarifa_returns_bad_request() {
    let pool = create_test_pool().await;

    let csv = "id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo\n\
               1;1;1.0;99.90;2024-01-01;2024-01-01;Ativo;Normal";
    let (status, body) = post_tarifas_csv(pool, "tarifas-sem-id.csv", csv).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(
        body["erro"],
        "O arquivo CSV de tarifas deve conter a coluna id_tarifa."
    );
}

#[actix_rt::test]
async fn test_import_tarifas_csv_inserts_new_rows() {
    let pool = create_test_pool().await;
    reset_tarifas(&pool, &[110001]).await;

    let csv = "id_tarifa;id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo\n\
               110001;1;1;1.0;99.90;2024-01-01;2024-01-01;Ativo;Normal";
    let (status, body) = post_tarifas_csv(pool.clone(), "tarifas-insert.csv", csv).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["tarifas_inseridas"], 1);
    assert_eq!(body["tarifas_atualizadas"], 0);
    assert_eq!(body["tarifas_importadas"], 1);
    assert_eq!(tarifa_valor(&pool, 110001).await, 99.90);

    reset_tarifas(&pool, &[110001]).await;
}

#[actix_rt::test]
async fn test_import_tarifas_csv_updates_existing_rows() {
    let pool = create_test_pool().await;
    seed_tarifa(&pool, 110002, 10.0).await;

    let csv = "id_tarifa;id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo\n\
               110002;1;1;1.0;77.70;2024-01-01;2024-02-01;Ativo;Normal";
    let (status, body) = post_tarifas_csv(pool.clone(), "tarifas-update.csv", csv).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["tarifas_inseridas"], 0);
    assert_eq!(body["tarifas_atualizadas"], 1);
    assert_eq!(body["tarifas_importadas"], 1);
    assert_eq!(tarifa_valor(&pool, 110002).await, 77.70);

    reset_tarifas(&pool, &[110002]).await;
}

#[actix_rt::test]
async fn test_import_tarifas_csv_mixed_response_includes_all_counters() {
    let pool = create_test_pool().await;
    seed_tarifa(&pool, 110003, 10.0).await;
    reset_tarifas(&pool, &[110004]).await;

    let csv = "id_tarifa;id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo\n\
               110003;1;1;1.0;33.30;2024-01-01;2024-02-01;Ativo;Normal\n\
               110004;1;1;1.0;44.40;2024-01-01;2024-02-01;Ativo;Normal";
    let (status, body) = post_tarifas_csv(pool.clone(), "tarifas-mixed.csv", csv).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["tarifas_inseridas"], 1);
    assert_eq!(body["tarifas_atualizadas"], 1);
    assert_eq!(body["tarifas_importadas"], 2);
    assert_eq!(tarifa_valor(&pool, 110003).await, 33.30);
    assert_eq!(tarifa_valor(&pool, 110004).await, 44.40);

    reset_tarifas(&pool, &[110003, 110004]).await;
}

#[actix_rt::test]
async fn test_import_tarifas_csv_duplicate_id_tarifa_returns_bad_request() {
    let pool = create_test_pool().await;
    reset_tarifas(&pool, &[110005]).await;

    let csv = "id_tarifa;id_tipo_tarifa;id_pedagio;multiplicador;valor;data_criacao;data_atualizacao;situacao;tipo\n\
               110005;1;1;1.0;55.50;2024-01-01;2024-02-01;Ativo;Normal\n\
               110005;1;1;1.0;66.60;2024-01-01;2024-02-01;Ativo;Normal";
    let (status, body) = post_tarifas_csv(pool.clone(), "tarifas-duplicate.csv", csv).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["erro"], "O arquivo CSV contém id_tarifa duplicado.");
    assert!(body["detalhes"].as_str().unwrap().contains("110005"));

    reset_tarifas(&pool, &[110005]).await;
}

#[actix_rt::test]
async fn test_import_pedagios_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_pedagios_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/pedagios")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}",
        resp.status()
    );
}

// ==================== File Upload Tests ====================

#[actix_rt::test]
async fn test_index_returns_html_form() {
    let app =
        test::init_service(App::new().service(web::resource("/").route(web::get().to(index))))
            .await;

    let req = test::TestRequest::get().uri("/").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "Expected success status, got {:?}",
        resp.status()
    );

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("<form"), "Expected HTML form in response");
    assert!(
        body_str.contains("multipart/form-data"),
        "Expected multipart form encoding"
    );
}
