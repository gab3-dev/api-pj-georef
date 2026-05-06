//! Tests for Import endpoints
//!
//! Note: These tests require multipart form handling which is more complex to test.
//! The tests here focus on verifying the endpoints are reachable and handle errors gracefully.

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{http::StatusCode, test, web, App};
use jsonwebtoken::{encode, EncodingKey, Header};
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

#[actix_rt::test]
async fn test_import_operadoras_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_operadoras),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/importar-operadoras")
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
            .service(import_tarifas),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/importar-tarifas")
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
async fn test_import_pedagios_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_pedagios),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/importar-pedagios")
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
