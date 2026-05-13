use crate::auth::{AdminAutenticado, UsuarioAutenticado};
use crate::models::*;
use crate::pedagio::{db, Pedagio};

#[post("/api/create-pedagio")]
pub async fn create_pedagio(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_pedagio_response(data, pool).await
}

#[post("/api/pedagios")]
pub async fn create_pedagio_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
    create_pedagio_response(data, pool).await
}

async fn create_pedagio_response(data: String, pool: web::Data<Pool>) -> HttpResponse {
    let pedagio = match Pedagio::new(data) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::insert(&pedagio, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Pedagio inserida com sucesso"),
        Err(err) => err,
    }
}

#[get("/api/get-pedagios")]
pub async fn get_all_pedagio(_user: UsuarioAutenticado, pool: web::Data<Pool>) -> impl Responder {
    get_all_pedagio_response(pool).await
}

#[get("/api/pedagios")]
pub async fn get_all_pedagio_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    get_all_pedagio_response(pool).await
}

async fn get_all_pedagio_response(pool: web::Data<Pool>) -> HttpResponse {
    match db::get_all(pool.get_ref()).await {
        Ok(pedagios) => HttpResponse::Ok().json(pedagios),
        Err(err) => err,
    }
}

#[get("/api/get-pedagio/{codigo_pedagio}")]
pub async fn get_pedagio_by_id(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    codigo_pedagio: web::Path<i8>,
) -> impl Responder {
    get_pedagio_by_id_response(pool, codigo_pedagio.into_inner()).await
}

#[get("/api/pedagios/{codigo_pedagio}")]
pub async fn get_pedagio_by_id_rest(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    codigo_pedagio: web::Path<i8>,
) -> impl Responder {
    get_pedagio_by_id_response(pool, codigo_pedagio.into_inner()).await
}

async fn get_pedagio_by_id_response(pool: web::Data<Pool>, codigo_pedagio: i8) -> HttpResponse {
    match db::get_by_id(pool.get_ref(), codigo_pedagio).await {
        Ok(pedagios) => HttpResponse::Ok().json(pedagios),
        Err(err) => err,
    }
}

#[put("/api/update-pedagio/{codigo_pedagio}")]
pub async fn update_pedagio(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    codigo_pedagio: web::Path<String>,
) -> impl Responder {
    update_pedagio_response(data, pool, codigo_pedagio.into_inner()).await
}

#[put("/api/pedagios/{codigo_pedagio}")]
pub async fn update_pedagio_rest(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
    codigo_pedagio: web::Path<String>,
) -> impl Responder {
    update_pedagio_response(data, pool, codigo_pedagio.into_inner()).await
}

async fn update_pedagio_response(
    data: String,
    pool: web::Data<Pool>,
    codigo_pedagio: String,
) -> HttpResponse {
    let pedagio = match Pedagio::new(data) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::update(&pedagio, codigo_pedagio, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Pedagio atualizado com sucesso"),
        Err(err) => err,
    }
}
