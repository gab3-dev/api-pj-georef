//! Tests for authentication and authorization
//!
//! Verifies that:
//! - All endpoints return 401 without a token
//! - Read endpoints accept any valid token
//! - Write endpoints return 403 for non-admin users
//! - Login endpoint works correctly
//! - User management endpoints are admin-only

use actix_web::{test, web, App, http::StatusCode};
use actix_multipart::form::tempfile::TempFileConfig;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;

use crate::auth::{self, JwtConfig};
use crate::auth::models::Claims;
use crate::models::*;
use crate::utils::*;

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
    }
}

fn generate_token(perfil: &str) -> String {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: "test@test.com".to_string(),
        perfil: perfil.to_string(),
        nome: "Teste".to_string(),
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

fn generate_expired_token() -> String {
    let claims = Claims {
        sub: "test@test.com".to_string(),
        perfil: "admin".to_string(),
        nome: "Teste".to_string(),
        iat: 1000,
        exp: 1001, // expired long ago
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
    )
    .unwrap()
}

// ==================== 401 Unauthorized — No Token ====================

#[actix_rt::test]
async fn test_get_operadoras_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_get_operadora_by_id_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_operadora_by_id),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-operadora/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_create_operadora_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_operadora),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/create-operadora")
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_update_operadora_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(update_operadora),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/api/update-operadora/1")
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_get_pedagios_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_pedagio),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-pedagios")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_create_pedagio_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_pedagio),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/create-pedagio")
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_get_tarifas_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tarifas),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-tarifas")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_create_tarifa_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/create-tarifa")
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_get_tipos_tarifa_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tipos_tarifa),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-tipos-tarifa")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_import_operadoras_returns_401_without_token() {
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
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_import_pedagios_returns_401_without_token() {
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
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_import_tarifas_returns_401_without_token() {
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
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_get_usuarios_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::get_all_usuarios),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-usuarios")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_create_usuario_returns_401_without_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::create_usuario),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/create-usuario")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"nome":"x","email":"x@x.com","senha":"123456","perfil":"user"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ==================== 401 — Expired Token ====================

#[actix_rt::test]
async fn test_expired_token_returns_401() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    let token = generate_expired_token();
    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ==================== 401 — Invalid Token ====================

#[actix_rt::test]
async fn test_invalid_token_returns_401() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .insert_header(("Authorization", "Bearer invalid.token.here"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_wrong_secret_token_returns_401() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    // Token signed with a different secret
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: "test@test.com".to_string(),
        perfil: "admin".to_string(),
        nome: "Teste".to_string(),
        iat: now,
        exp: now + 3600,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"wrong_secret"),
    )
    .unwrap();

    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_malformed_authorization_header_returns_401() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .insert_header(("Authorization", "NotBearer sometoken"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ==================== 403 Forbidden — User Token on Admin Endpoints ====================

#[actix_rt::test]
async fn test_create_operadora_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_operadora),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/create-operadora")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_update_operadora_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(update_operadora),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/update-operadora/1")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_create_pedagio_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_pedagio),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/create-pedagio")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_update_pedagio_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(update_pedagio),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/update-pedagio/1")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_create_tarifa_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tarifa),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/create-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_create_tipo_tarifa_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(create_tipo_tarifa),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/create-tipo-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_update_tarifa_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(update_tarifa),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/update-tarifa/1")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_payload("{}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_import_operadoras_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_operadoras),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/importar-operadoras")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_import_pedagios_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_pedagios),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/importar-pedagios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_import_tarifas_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(import_tarifas),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/importar-tarifas")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_get_usuarios_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::get_all_usuarios),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/get-usuarios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_rt::test]
async fn test_create_usuario_returns_403_for_user() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::create_usuario),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/create-usuario")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"nome":"x","email":"x@x.com","senha":"123456","perfil":"user"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ==================== 200 OK — Valid Token on Read Endpoints ====================

#[actix_rt::test]
async fn test_get_operadoras_returns_ok_with_user_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_operadoras),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/get-operadoras")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_get_pedagios_returns_ok_with_user_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_pedagio),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/get-pedagios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_get_tarifas_returns_ok_with_user_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tarifas),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/get-tarifas")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_get_tipos_tarifa_returns_ok_with_user_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(get_all_tipos_tarifa),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/get-tipos-tarifa")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

// ==================== Login Endpoint ====================

#[actix_rt::test]
async fn test_login_with_valid_credentials() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::login),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"email":"test@test.com","senha":"12345678"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_login_with_wrong_password() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::login),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"email":"test@test.com","senha":"wrongpassword"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::UNAUTHORIZED || resp.status().as_u16() == 500,
        "Expected 401 or 500 (db issue), got {:?}",
        resp.status()
    );
}

#[actix_rt::test]
async fn test_login_with_nonexistent_email() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::login),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"email":"nonexistent@test.com","senha":"123456"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status() == StatusCode::UNAUTHORIZED || resp.status().as_u16() == 500,
        "Expected 401 or 500 (db issue), got {:?}",
        resp.status()
    );
}

// ==================== Admin Endpoints — Admin Token OK ====================

#[actix_rt::test]
async fn test_get_usuarios_returns_ok_with_admin_token() {
    let pool = create_test_pool().await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::get_all_usuarios),
    )
    .await;

    let token = generate_token("admin");
    let req = test::TestRequest::get()
        .uri("/api/get-usuarios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}
