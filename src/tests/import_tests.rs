//! Tests for Import endpoints
//!
//! Note: These tests require multipart form handling which is more complex to test.
//! The tests here focus on verifying the endpoints are reachable and handle errors gracefully.

use actix_web::{test, web, App, http::StatusCode};
use actix_multipart::form::tempfile::TempFileConfig;
use deadpool_postgres::{Config, PoolConfig, Runtime};
use jsonwebtoken::{encode, EncodingKey, Header};
use tokio_postgres::NoTls;

use crate::auth::JwtConfig;
use crate::auth::models::Claims;
use crate::utils::*;

const TEST_SECRET: &str = "test_secret_key";

async fn create_test_pool() -> deadpool_postgres::Pool {
    let mut cfg = Config::new();
    cfg.host = Some(std::env::var("DB_HOST").unwrap_or("localhost".into()));
    cfg.port = Some(5432);
    cfg.dbname = Some("pj_georef".to_string());
    cfg.user = Some("root".to_string());
    cfg.password = Some("1234".to_string());
    cfg.pool = PoolConfig::new(5).into();
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

fn jwt_config() -> JwtConfig {
    JwtConfig { secret: TEST_SECRET.to_string() }
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
    encode(&Header::default(), &claims, &EncodingKey::from_secret(TEST_SECRET.as_bytes())).unwrap()
}

#[actix_rt::test]
async fn test_import_operadoras_without_file_returns_error() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_operadoras)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/importar-operadoras")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}", resp.status()
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
            .service(import_tarifas)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/importar-tarifas")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}", resp.status()
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
            .service(import_pedagios)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/importar-pedagios")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::BAD_REQUEST || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected error status for missing file, got {:?}", resp.status()
    );
}

// ==================== File Upload Tests ====================

#[actix_rt::test]
async fn test_index_returns_html_form() {
    let app = test::init_service(
        App::new()
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "Expected success status, got {:?}", resp.status());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("<form"), "Expected HTML form in response");
    assert!(body_str.contains("multipart/form-data"), "Expected multipart form encoding");
}
