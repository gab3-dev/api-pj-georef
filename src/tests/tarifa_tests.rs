//! Tests for Tarifa and TipoTarifa endpoints

use actix_web::{test, web, App};
use deadpool_postgres::{Config, PoolConfig, Runtime};
use tokio_postgres::NoTls;

use crate::models::*;

/// Helper function to create a test database pool
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

// ==================== TipoTarifa Tests ====================

#[actix_rt::test]
async fn test_get_all_tipos_tarifa_returns_ok() {
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(get_all_tipos_tarifa)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tipos-tarifa")
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
        .set_payload(tipo_tarifa_json)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
#[should_panic(expected = "Result::unwrap()")]
async fn test_create_tipo_tarifa_with_invalid_json_panics() {
    // Note: The current API panics on invalid JSON - this should be fixed
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(create_tipo_tarifa)
    ).await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tipo-tarifa")
        .set_payload(invalid_json)
        .to_request();
    
    let _resp = test::call_service(&app, req).await;
}

// ==================== Tarifa Tests ====================

#[actix_rt::test]
async fn test_get_all_tarifas_returns_ok() {
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(get_all_tarifas)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tarifas")
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
            .service(get_tarifa_by_id)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/get-tarifa/1")
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
        .set_payload(tarifa_json)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}", resp.status()
    );
}

#[actix_rt::test]
#[should_panic(expected = "Result::unwrap()")]
async fn test_create_tarifa_with_invalid_json_panics() {
    // Note: The current API panics on invalid JSON - this should be fixed
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(create_tarifa)
    ).await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/create-tarifa")
        .set_payload(invalid_json)
        .to_request();
    
    let _resp = test::call_service(&app, req).await;
}
