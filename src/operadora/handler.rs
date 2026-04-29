use crate::auth::{AdminAutenticado, UsuarioAutenticado};
use crate::models::*;
use crate::operadora::{db, Operadora};

#[post("/api/create-operadora")]
pub async fn create_operadora(
    _admin: AdminAutenticado,
    data: String,
    pool: web::Data<Pool>,
) -> impl Responder {
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
    match db::get_all(pool.get_ref()).await {
        Ok(rows) => {
            let operadoras: Vec<Operadora> = rows.iter().map(Operadora::from).collect();
            HttpResponse::Ok().json(operadoras)
        }
        Err(err) => err,
    }
}

#[get("/api/get-operadora/{codigo_operadora}")]
pub async fn get_operadora_by_id(
    _user: UsuarioAutenticado,
    pool: web::Data<Pool>,
    codigo_operadora: web::Path<i32>,
) -> impl Responder {
    match db::get_by_id(pool.get_ref(), codigo_operadora.into_inner()).await {
        Ok(rows) => {
            let operadoras: Vec<Operadora> = rows.iter().map(Operadora::from).collect();
            HttpResponse::Ok().json(operadoras)
        }
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
    let operadora = match Operadora::new(data) {
        Ok(o) => o,
        Err(e) => return HttpResponse::BadRequest().body(format!("JSON inválido: {}", e)),
    };

    match db::update(&operadora, codigo_operadora.into_inner(), pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("Operadora atualizada com sucesso"),
        Err(err) => err,
    }
}
