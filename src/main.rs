use actix_web::{web, App, HttpServer, http::KeepAlive};
use deadpool_postgres::{Config, PoolConfig, Runtime};
use tokio_postgres::NoTls;
use std::{env, sync::Mutex};

mod db;
use db::*;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    dbg!("{counter}");
    
    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let mut cfg = Config::new();
    cfg.host = Some(
        env::var("DB_HOST")
            .unwrap_or("localhost".into())
            .to_string(),
    );
    cfg.port = Some(5432);
    cfg.dbname = Some("pj-georef".to_string());
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
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .app_data(pool.clone())
            .service(create_operadora)
            .service(create_praca)
            .route("/", web::get().to(index))
    })
    .keep_alive(KeepAlive::Os)
    .bind(format!("0.0.0.0:{http_port}"))?
    .run()
    .await

}
