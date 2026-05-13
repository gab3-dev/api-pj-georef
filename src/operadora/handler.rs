use crate::auth::{AdminAutenticado, UsuarioAutenticado};
use crate::models::*;
use crate::operadora::{db, Operadora};

#[post("/api/create-operadora")]
pub async fn create_operadora(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_operadora_response(data, pool).await
}

#[post("/api/operadoras")]
pub async fn create_operadora_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_operadora_response(data, pool).await
}

async fn create_operadora_response(data: String, pool: web::Data<Pool>) -> HttpResponse {
    let operadora = match Operadora::new(data) {
        Ok(o) => o,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::insert(&operadora, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Operadora inserida com sucesso"),
        Err(err) => err,
    }
}

#[get("/api/get-operadoras")]
pub async fn get_all_operadoras(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_operadoras_response(pool).await
}

#[get("/api/operadoras")]
pub async fn get_all_operadoras_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_operadoras_response(pool).await
}

async fn get_all_operadoras_response(pool: web::Data<Pool>) -> HttpResponse {
    match db::get_all(pool.get_ref()).await {
        Ok(operadoras) => HttpResponse::Ok().json(operadoras),
        Err(err) => err,
    }
}

#[get("/api/get-operadora/{codigo_operadora}")]
pub async fn get_operadora_by_id(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    get_operadora_by_id_response(pool, codigo_operadora.into_inner()).await
}

#[get("/api/operadoras/{codigo_operadora}")]
pub async fn get_operadora_by_id_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    get_operadora_by_id_response(pool, codigo_operadora.into_inner()).await
}

async fn get_operadora_by_id_response(
    pool: web::Data<Pool>,
    codigo_operadora: i32,
) -> HttpResponse {
    match db::get_by_id(pool.get_ref(), codigo_operadora).await {
        Ok(operadoras) => HttpResponse::Ok().json(operadoras),
        Err(err) => err,
    }
}

#[put("/api/update-operadora/{codigo_operadora}")]
pub async fn update_operadora(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    update_operadora_response(data, pool, codigo_operadora.into_inner()).await
}

#[put("/api/operadoras/{codigo_operadora}")]
pub async fn update_operadora_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    update_operadora_response(data, pool, codigo_operadora.into_inner()).await
}

async fn update_operadora_response(
    data: String,
    pool: web::Data<Pool>,
    codigo_operadora: i32,
) -> HttpResponse {
    let operadora = match Operadora::new(data) {
        Ok(o) => o,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::update(&operadora, codigo_operadora, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Operadora atualizada com sucesso"),
        Err(err) => err,
    }
}
