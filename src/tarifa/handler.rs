use crate::auth::{AdminAutenticado, UsuarioAutenticado};
use crate::models::*;
use crate::tarifa::{db, Tarifa, TipoTarifa};

#[post("/api/create-tipo-tarifa")]
pub async fn create_tipo_tarifa(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_tipo_tarifa_response(data, pool).await
}

#[post("/api/tipos-tarifa")]
pub async fn create_tipo_tarifa_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_tipo_tarifa_response(data, pool).await
}

async fn create_tipo_tarifa_response(data: String, pool: web::Data<Pool>) -> HttpResponse {
    let tipo_tarifa = match TipoTarifa::new(data) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::insert_tipo_tarifa(&tipo_tarifa, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Tipo de tarifa inserida com sucesso"),
        Err(err) => err,
    }
}

#[get("/api/get-tipos-tarifa")]
pub async fn get_all_tipos_tarifa(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_tipos_tarifa_response(pool).await
}

#[get("/api/tipos-tarifa")]
pub async fn get_all_tipos_tarifa_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_tipos_tarifa_response(pool).await
}

async fn get_all_tipos_tarifa_response(pool: web::Data<Pool>) -> HttpResponse {
    match db::get_all_tipos_tarifa(pool.get_ref()).await {
        Ok(tipos_tarifa) => HttpResponse::Ok().json(tipos_tarifa),
        Err(err) => err,
    }
}

#[post("/api/create-tarifa")]
pub async fn create_tarifa(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_tarifa_response(data, pool).await
}

#[post("/api/tarifas")]
pub async fn create_tarifa_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_tarifa_response(data, pool).await
}

async fn create_tarifa_response(data: String, pool: web::Data<Pool>) -> HttpResponse {
    let tarifa = match Tarifa::new(data) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::insert_tarifa(&tarifa, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Tarifa inserida com sucesso"),
        Err(err) => err,
    }
}

#[get("/api/get-tarifas")]
pub async fn get_all_tarifas(_user: UsuarioAutenticado, pool: web::Data<Pool>) -> impl Responder {
    get_all_tarifas_response(pool).await
}

#[get("/api/tarifas")]
pub async fn get_all_tarifas_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_tarifas_response(pool).await
}

async fn get_all_tarifas_response(pool: web::Data<Pool>) -> HttpResponse {
    match db::get_all_tarifas(pool.get_ref()).await {
        Ok(tarifas) => HttpResponse::Ok().json(tarifas),
        Err(err) => err,
    }
}

#[get("/api/get-tarifa/{id_tarifa}")]
pub async fn get_tarifa_by_id(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    id_tarifa: web::Path<i32>,
) -> impl Responder {
    get_tarifa_by_id_response(pool, id_tarifa.into_inner()).await
}

#[get("/api/tarifas/{id_tarifa}")]
pub async fn get_tarifa_by_id_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    id_tarifa: web::Path<i32>,
) -> impl Responder {
    get_tarifa_by_id_response(pool, id_tarifa.into_inner()).await
}

async fn get_tarifa_by_id_response(pool: web::Data<Pool>, id_tarifa: i32) -> HttpResponse {
    match db::get_tarifa_by_id(pool.get_ref(), id_tarifa).await {
        Ok(tarifas) => HttpResponse::Ok().json(tarifas),
        Err(err) => err,
    }
}

#[put("/api/update-tarifa/{id_tarifa}")]
pub async fn update_tarifa(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    id_tarifa: web::Path<i32>,
) -> impl Responder {
    update_tarifa_response(data, pool, id_tarifa.into_inner()).await
}

#[put("/api/tarifas/{id_tarifa}")]
pub async fn update_tarifa_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    id_tarifa: web::Path<i32>,
) -> impl Responder {
    update_tarifa_response(data, pool, id_tarifa.into_inner()).await
}

async fn update_tarifa_response(
    data: String,
    pool: web::Data<Pool>,
    id_tarifa: i32,
) -> HttpResponse {
    let tarifa = match Tarifa::new(data) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::update_tarifa(&tarifa, id_tarifa, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Tarifa atualizada com sucesso"),
        Err(err) => err,
    }
}
