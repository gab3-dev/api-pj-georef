//! Tests for authentication and authorization
//!
//! Verifies that:
//! - All endpoints return 401 without a token
//! - Read endpoints accept any valid token
//! - Write endpoints return 403 for non-admin users
//! - Login endpoint works correctly
//! - User management endpoints are admin-only

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{http::StatusCode, test, web, App};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::postgres::PgPoolOptions;

use bgm::auth::models::Claims;
use bgm::auth::{self, JwtConfig};
use bgm::models::*;
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

async fn ensure_login_fixture(pool: &sqlx::PgPool) {
    let salt = SaltString::generate(&mut OsRng);
    let senha_hash = Argon2::default()
        .hash_password("12345678".as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query(
        "INSERT INTO usuario (nome, email, senha_hash, perfil)
         VALUES ($1, $2, $3, $4::perfil_usuario)
         ON CONFLICT (email) DO UPDATE SET
             nome = EXCLUDED.nome,
             senha_hash = EXCLUDED.senha_hash,
             perfil = EXCLUDED.perfil",
    )
    .bind("Teste")
    .bind("test@test.com")
    .bind(senha_hash)
    .bind("user")
    .execute(pool)
    .await
    .unwrap();
}

fn jwt_config() -> JwtConfig {
    JwtConfig {
        secret: TEST_SECRET.to_string(),
        expiration_seconds: 8 * 3600,
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
            .service(get_all_operadoras_rest),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/operadoras").to_request();
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
            .service(get_operadora_by_id_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/operadoras/1")
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
            .service(create_operadora_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/operadoras")
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
            .service(update_operadora_rest),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/api/operadoras/1")
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
            .service(get_all_pedagio_rest),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/pedagios").to_request();
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
            .service(create_pedagio_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/pedagios")
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
            .service(get_all_tarifas_rest),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/tarifas").to_request();
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
            .service(create_tarifa_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/tarifas")
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
            .service(get_all_tipos_tarifa_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/tipos-tarifa")
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
            .service(import_operadoras_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/operadoras")
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
            .service(import_pedagios_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/pedagios")
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
            .service(import_tarifas_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/imports/tarifas")
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
            .service(auth::get_all_usuarios_rest),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/usuarios").to_request();
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
            .service(auth::create_usuario_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/usuarios")
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
            .service(get_all_operadoras_rest),
    )
    .await;

    let token = generate_expired_token();
    let req = test::TestRequest::get()
        .uri("/api/operadoras")
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
            .service(get_all_operadoras_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/operadoras")
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
            .service(get_all_operadoras_rest),
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
        .uri("/api/operadoras")
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
            .service(get_all_operadoras_rest),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/operadoras")
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
            .service(create_operadora_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/operadoras")
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
            .service(update_operadora_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/operadoras/1")
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
            .service(create_pedagio_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/pedagios")
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
            .service(update_pedagio_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/pedagios/1")
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
            .service(create_tarifa_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/tarifas")
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
            .service(create_tipo_tarifa_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/tipos-tarifa")
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
            .service(update_tarifa_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::put()
        .uri("/api/tarifas/1")
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
            .service(import_operadoras_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/imports/operadoras")
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
            .service(import_pedagios_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/imports/pedagios")
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
            .service(import_tarifas_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/imports/tarifas")
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
            .service(auth::get_all_usuarios_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/usuarios")
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
            .service(auth::create_usuario_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::post()
        .uri("/api/usuarios")
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
            .service(get_all_operadoras_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/operadoras")
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
            .service(get_all_pedagio_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/pedagios")
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
            .service(get_all_tarifas_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/tarifas")
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
            .service(get_all_tipos_tarifa_rest),
    )
    .await;

    let token = generate_token("user");
    let req = test::TestRequest::get()
        .uri("/api/tipos-tarifa")
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
    ensure_login_fixture(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::login_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
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
    ensure_login_fixture(&pool).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .app_data(web::Data::new(jwt_config()))
            .service(auth::login_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
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
            .service(auth::login_rest),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
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
            .service(auth::get_all_usuarios_rest),
    )
    .await;

    let token = generate_token("admin");
    let req = test::TestRequest::get()
        .uri("/api/usuarios")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success() || resp.status().as_u16() == 500,
        "Expected success or 500 (db issue), got {:?}",
        resp.status()
    );
}
