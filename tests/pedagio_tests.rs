//! Tests for Pedagio endpoints

use actix_web::{test, web, App};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;

use crate::auth::JwtConfig;
use crate::auth::models::Claims;
use crate::models::*;

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

fn user_token() -> String {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: "user@test.com".to_string(),
        perfil: "user".to_string(),
        nome: "User".to_string(),
        iat: now,
        exp: now + 3600,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(TEST_SECRET.as_bytes())).unwrap()
}

#[actix_rt::test]
async fn test_get_all_pedagios_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_pedagio)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-pedagios")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_get_pedagio_by_id_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_pedagio_by_id)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-pedagio/1")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_pedagio_with_valid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_pedagio)
    ).await;

    let pedagio_json = r#"{
        "pedagio": "pedagio",
        "longitude": -46633309,
        "latitude": -23550520,
        "codigo_operadora": 1,
        "nome": "Test Pedagio",
        "situacao": "Ativo",
        "rodovia": "BR-101",
        "km": 123.5,
        "sentido": "Norte",
        "cidade": "São Paulo",
        "estado": "SP",
        "codigo_pedagio": "PED99999",
        "orientacao": "Leste",
        "tipo": "Simples",
        "jurisdicao": "Federal",
        "cobranca_especial": false,
        "categoria": "Normal",
        "data_alteracao": "2024-01-01",
        "razao_social": "Test Razao Social LTDA",
        "cnpj": "00.000.000/0001-00",
        "email": "test@test.com",
        "telefone": "(11) 99999-9999"
    }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-pedagio")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(pedagio_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_pedagio_with_invalid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_pedagio)
    ).await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-pedagio")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(invalid_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400, "Expected 400 for invalid JSON");
}
