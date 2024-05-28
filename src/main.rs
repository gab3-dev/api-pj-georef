use actix_web::{http, http::KeepAlive, web::{self}, App, HttpServer};
use actix_cors::Cors;
use deadpool_postgres::{Config, PoolConfig, Runtime};
use tokio_postgres::NoTls;
use std::env;

mod db;
use db::*;
mod files;
use files::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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

    let http_port = env::var("HTTP_PORT").unwrap_or("80".into());

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new() // <- register the created data
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(create_operadora)
            .service(create_praca)
            .service(upload_stats)
            .route("/", web::get().to(|| async { "Hello, world!" }))
    })
    .keep_alive(KeepAlive::Os)
    .bind(format!("0.0.0.0:{http_port}"))?
    .run()
    .await?;

    Ok(())

}
