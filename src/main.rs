use actix_cors::Cors;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    http,
    http::KeepAlive,
    middleware,
    web::{self},
    App, HttpServer,
};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod auth;
mod operadora;
mod pedagio;
mod models;
mod tarifa;
use models::*;
mod utils;
use utils::*;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    let db_host = env::var("DB_HOST").unwrap_or("localhost".into());
    let db_port = env::var("DB_PORT").unwrap_or("5432".into());
    let db_name = env::var("DB_NAME").unwrap_or("pj_georef".into());
    let db_user = env::var("DB_USER").unwrap_or("root".into());
    let db_password = env::var("DB_PASSWORD").unwrap_or("1234".into());
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

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

    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or("chave_secreta_desenvolvimento".into());
    let jwt_config = auth::JwtConfig { secret: jwt_secret };

    let http_port = env::var("HTTP_PORT").unwrap_or("9999".into());

    log::info!("Running on port {http_port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .block_on_origin_mismatch(false)
            .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(jwt_config.clone()))
            .service(auth::login)
            .service(auth::create_usuario)
            .service(auth::get_all_usuarios)
            .service(create_operadora)
            .service(create_pedagio)
            .service(get_all_operadoras)
            .service(get_all_pedagio)
            .service(get_operadora_by_id)
            .service(get_pedagio_by_id)
            .service(create_tipo_tarifa)
            .service(get_all_tipos_tarifa)
            .service(get_tarifa_by_id)
            .service(create_tarifa)
            .service(get_all_tarifas)
            .service(get_tarifa_by_id)
            .service(import_tarifas)
            .service(import_operadoras)
            .service(import_pedagios)
            .service(update_operadora)
            .service(update_pedagio)
            .service(update_tarifa)
            .service(                web::resource("/")
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
