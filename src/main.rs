use actix_cors::Cors;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    http,
    http::KeepAlive,
    middleware,
    web::{self},
    App, HttpServer,
};
use deadpool_postgres::{Config, PoolConfig, Runtime};
use std::env;
use tokio_postgres::NoTls;

mod models;
use models::*;
mod utils;
use utils::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    let mut cfg = Config::new();
    cfg.host = Some(
        env::var("DB_HOST")
            .unwrap_or("localhost".into())
            .to_string(),
    );
    cfg.port = Some(5432);
    cfg.dbname = Some("pj_georef".to_string());
    cfg.user = Some("root".to_string());
    cfg.password = Some("1234".to_string());

    let pool_size = env::var("POOL_SIZE")
        .unwrap_or("125".to_string())
        .parse::<usize>()
        .unwrap();

    cfg.pool = PoolConfig::new(pool_size).into();
    println!("creating postgres pool...");
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    println!("postgres pool succesfully created");

    let http_port = env::var("HTTP_PORT").unwrap_or("6969".into());

    log::info!("Running on port {http_port}");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .block_on_origin_mismatch(false)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new() // <- register the created data
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(pool.clone()))
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
