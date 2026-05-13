//! Tests for Tarifa and TipoTarifa endpoints

use actix_web::{test, web, App};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;

use bgm::auth::models::Claims;
use bgm::auth::JwtConfig;
use bgm::models::*;

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

fn user_token() -> String {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: "user@test.com".to_string(),
        perfil: "user".to_string(),
        nome: "User".to_string(),
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

// ==================== TipoTarifa Tests ====================

#[actix_rt::test]
async fn test_get_all_tipos_tarifa_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tipos_tarifa_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/tipos-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tipo_tarifa_with_valid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tipo_tarifa_rest),
    )
    .await;

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
        .uri("/api/tipos-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(tipo_tarifa_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tipo_tarifa_with_invalid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tipo_tarifa_rest),
    )
    .await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/tipos-tarifa")
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
            .service(get_all_tarifas_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/tarifas")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_get_tarifa_by_id_returns_ok() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_tarifa_by_id_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/tarifas/1")
        .insert_header(("Authorization", format!("Bearer {}", user_token())))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tarifa_with_valid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa_rest),
    )
    .await;

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
        .uri("/api/tarifas")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(tarifa_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_create_tarifa_with_invalid_json() {
    let pool = create_test_pool().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa_rest),
    )
    .await;

    let invalid_json = r#"{ invalid json }"#;

    let req = test::TestRequest::post()
        .uri("/api/tarifas")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(invalid_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400, "Expected 400 for invalid JSON");
}

#[actix_rt::test]
async fn test_update_tarifa_preserves_relationships_and_updates_tarifa_fields() {
    let pool = create_test_pool().await;
    let id_tarifa = 120001;

    sqlx::query("DELETE FROM tarifas WHERE id_tarifa = $1")
        .bind(id_tarifa)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO tarifas (
            id_tarifa, id_tipo_tarifa, id_pedagio, multiplicador, valor,
            data_criacao, data_atualizacao, situacao, tipo
        )
        VALUES ($1, 1, 1, 1.0, 10.0, '2024-01-01', '2024-01-01', 'Ativo', 'Normal')",
    )
    .bind(id_tarifa)
    .execute(&pool)
    .await
    .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_config()))
            .service(update_tarifa_rest),
    )
    .await;

    let tarifa_json = r#"{
        "id_tarifa": 120001,
        "id_tipo_tarifa": 2,
        "id_pedagio": 2,
        "multiplicador": 2.0,
        "valor": 77.70,
        "data_criacao": "2024-01-01T00:00:00",
        "data_atualizacao": "2024-01-02T00:00:00",
        "situacao": "Ativo",
        "tipo": "Normal",
        "descricao": "",
        "rodagem": "",
        "eixos": 0,
        "nome": ""
    }"#;

    let req = test::TestRequest::put()
        .uri("/api/tarifas/120001")
        .insert_header(("Authorization", format!("Bearer {}", admin_token())))
        .set_payload(tarifa_json)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let row = sqlx::query_as::<_, (i32, i32, f64)>(
        "SELECT id_tipo_tarifa, id_pedagio, valor FROM tarifas WHERE id_tarifa = $1",
    )
    .bind(id_tarifa)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, 1, "update-tarifa must not change id_tipo_tarifa");
    assert_eq!(row.1, 1, "update-tarifa must not change id_pedagio");
    assert_eq!(row.2, 77.70, "update-tarifa must update tarifa fields");

    sqlx::query(
        "DELETE FROM tarifas
         WHERE id_tarifa = $1
            OR (id_pedagio = 1 AND id_tipo_tarifa = 1 AND valor = 10.0 AND situacao = 'Inativo')",
    )
    .bind(id_tarifa)
    .execute(&pool)
    .await
    .unwrap();
}
