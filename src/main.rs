use actix_cors::Cors;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    http,
    http::KeepAlive,
    middleware,
    web::{self},
    App, HttpServer,
};
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::{env, io};

use bgm::{auth, models::*, utils::*};

fn is_production() -> bool {
    matches!(
        env::var("APP_ENV")
            .or_else(|_| env::var("RUST_ENV"))
            .unwrap_or_default()
            .to_lowercase()
            .as_str(),
        "production" | "prod"
    )
}

fn env_required(name: &str) -> io::Result<String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => Ok(value),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{name} must be set"),
        )),
    }
}

fn env_with_default(name: &str, default: &str) -> io::Result<String> {
    if is_production() {
        env_required(name)
    } else {
        Ok(env::var(name).unwrap_or_else(|_| default.to_string()))
    }
}

fn build_cors(allowed_origins: &str) -> Cors {
    let origins: Vec<&str> = allowed_origins
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .collect();

    let cors = if origins.iter().any(|origin| *origin == "*") {
        Cors::default().send_wildcard().allow_any_origin()
    } else {
        let mut cors = Cors::default();
        for origin in origins {
            cors = cors.allowed_origin(origin);
        }
        cors
    };

    cors.block_on_origin_mismatch(false)
        .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("creating temporary upload directory");
    let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./tmp".to_string());
    std::fs::create_dir_all(&upload_dir)?;

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => {
            let db_host = env_with_default("DB_HOST", "localhost")?;
            let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            let db_name = env_with_default("DB_NAME", "pj_georef")?;
            let db_user = env_with_default("DB_USER", "root")?;
            let db_password = env_with_default("DB_PASSWORD", "1234")?;

            format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            )
        }
    };

    let pool_size = env::var("POOL_SIZE")
        .unwrap_or("125".to_string())
        .parse::<usize>()
        .unwrap();

    println!("creating postgres pool...");
    let pool = PgPoolOptions::new()
        .max_connections(pool_size as u32)
        .connect(&database_url)
        .await
        .unwrap();
    println!("postgres pool succesfully created");

    auth::seed_admin(&pool).await;

    let jwt_secret = env_with_default("JWT_SECRET", "chave_secreta_desenvolvimento")?;
    let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let jwt_config = auth::JwtConfig {
        secret: jwt_secret,
        expiration_seconds: jwt_expiration_hours * 3600,
    };

    let http_port = env::var("HTTP_PORT").unwrap_or("9999".into());
    let cors_allowed_origins = if is_production() {
        env_required("CORS_ALLOWED_ORIGINS")?
    } else {
        env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string())
    };

    log::info!("Running on port {http_port}");

    HttpServer::new(move || {
        let cors = build_cors(&cors_allowed_origins);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory(upload_dir.clone()))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_config.clone()))
            .service(auth::login)
            .service(auth::login_rest)
            .service(auth::create_usuario)
            .service(auth::create_usuario_rest)
            .service(auth::get_all_usuarios)
            .service(auth::get_all_usuarios_rest)
            .service(create_operadora)
            .service(create_operadora_rest)
            .service(create_pedagio)
            .service(create_pedagio_rest)
            .service(get_all_operadoras)
            .service(get_all_operadoras_rest)
            .service(get_all_pedagio)
            .service(get_all_pedagio_rest)
            .service(get_operadora_by_id)
            .service(get_operadora_by_id_rest)
            .service(get_pedagio_by_id)
            .service(get_pedagio_by_id_rest)
            .service(create_tipo_tarifa)
            .service(create_tipo_tarifa_rest)
            .service(get_all_tipos_tarifa)
            .service(get_all_tipos_tarifa_rest)
            .service(get_tarifa_by_id)
            .service(get_tarifa_by_id_rest)
            .service(create_tarifa)
            .service(create_tarifa_rest)
            .service(get_all_tarifas)
            .service(get_all_tarifas_rest)
            .service(import_tarifas)
            .service(import_tarifas_rest)
            .service(import_operadoras)
            .service(import_operadoras_rest)
            .service(import_pedagios)
            .service(import_pedagios_rest)
            .service(update_operadora)
            .service(update_operadora_rest)
            .service(update_pedagio)
            .service(update_pedagio_rest)
            .service(update_tarifa)
            .service(update_tarifa_rest)
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_files)),
            )
    })
    .keep_alive(KeepAlive::Os)
    .bind(format!("0.0.0.0:{http_port}"))?
    .run()
    .await?;

    Ok(())
}
