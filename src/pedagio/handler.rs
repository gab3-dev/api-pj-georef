use crate::auth::{AdminAutenticado, UsuarioAutenticado};
use crate::models::*;
use crate::pedagio::{db, Pedagio};

#[post("/api/create-pedagio")]
pub async fn create_pedagio(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
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
pub async fn get_all_pedagio(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
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
    match db::get_by_id(pool.get_ref(), codigo_pedagio.into_inner()).await {
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
    let pedagio = match Pedagio::new(data) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::update(&pedagio, codigo_pedagio.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Pedagio atualizado com sucesso"),
        Err(err) => err,
    }
}
