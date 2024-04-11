use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, http::KeepAlive};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "operadora")]
struct Operadora {
    id: String,
    data_operacao: String,
    responsavel: String,
    grupo: String,
    codigo_operadora: i32,
    operadora: String,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

#[post("/create")]
async fn create_operadora(data: String) -> impl Responder {
    let operadora = serde_json::to_string(&new_operadora(data)).unwrap();
    HttpResponse::Ok().body(operadora)
}

fn new_operadora(json: String) -> Operadora {
    println!("{}", json);
    let result: Operadora = serde_json::from_str(&json.as_str()).unwrap();

    return result;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "praca")]
struct Praca {
    id: String,
    longitude: i32,
    latitude: i32,
    id_operadora: String,
    nome: String,
    situacao: String,
    rodovia: String,
    km: i32,
    sentido: String,
    cidade: String,
    estado: String,
    codigo_praca: i8,
    orientacao: String,
    tipo: String,
    jurisdicao: String,
    cobranca_especial: bool,
    categoria: String,
    data_de_alteracao: String,
    razao_social: String,
    cnpj: String,
    email: String,
    telefone: String,
}

#[post("/create")]
async fn create_praca(data: String) -> impl Responder {
    let praca = serde_json::to_string(&new_praca(data)).unwrap();
    HttpResponse::Ok().body(praca)
}

fn new_praca(json: String) -> Praca {
    println!("{}", json);
    let result: Praca = serde_json::from_str(&json.as_str()).unwrap();

    return result;
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

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .service(create_operadora)
            .service(create_praca)
            .route("/", web::get().to(index))
    })
    .keep_alive(KeepAlive::Os)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await

}
