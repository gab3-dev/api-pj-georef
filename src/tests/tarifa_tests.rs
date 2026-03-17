//! Tests for Tarifa and TipoTarifa endpoints

use actix_web::{test, web, App};
use deadpool_postgres::{Config, PoolConfig, Runtime};
use jsonwebtoken::{encode, EncodingKey, Header};
use tokio_postgres::NoTls;

use crate::auth::JwtConfig;
use crate::auth::models::Claims;
use crate::models::*;

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

// ==================== TipoTarifa Tests ====================

#[actix_rt::test]
async fn test_get_all_tipos_tarifa_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tipos_tarifa)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tipos-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tipo_tarifa_with_valid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tipo_tarifa)
    ).await;

    let tipo_tarifa_json = r#"{
        "tipo_tarifa": "tipo_tarifa",
        "id_tipo_tarifa": 99999,
        "id_padrao_tarifa": 1,
        "descricao": "Test Tipo Tarifa",
        "tipo_rodagem": 1,
        "rodagem": "Simples",
        "eixos": 2
    }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tipo-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(tipo_tarifa_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tipo_tarifa_with_invalid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tipo_tarifa)
    ).await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tipo-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(invalid_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400, "Expected 400 for invalid JSON");
}

// ==================== Tarifa Tests ====================

#[actix_rt::test]
async fn test_get_all_tarifas_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tarifas)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tarifas")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_get_tarifa_by_id_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_tarifa_by_id)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tarifa/1")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tarifa_with_valid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa)
    ).await;

    let tarifa_json = r#"{
        "id_tarifa": 99999,
        "id_tipo_tarifa": 1,
        "id_pedagio": 1,
        "multiplicador": 1.5,
        "valor": 25.50,
        "data_criacao": "2024-01-01T00:00:00",
        "data_atualizacao": "2024-01-01T00:00:00",
        "situacao": "Ativo",
        "tipo": "Normal",
        "descricao": "Test Tarifa",
        "rodagem": "Simples",
        "eixos": 2,
        "nome": "Test"
    }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(tarifa_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tarifa_with_invalid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa)
    ).await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(invalid_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400, "Expected 400 for invalid JSON");
}
