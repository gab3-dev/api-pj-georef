use actix_web::{post, get, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use deadpool_postgres::Pool;
use jsonwebtoken::{encode, EncodingKey, Header};

use super::middleware::AdminAutenticado;
use super::models::*;

#[post("/api/login")]
pub async fn login(
    data: web::Json<LoginRequest>,
    pool: web::Data<Pool>,
    jwt_config: web::Data<JwtConfig>,
) -> impl Responder {
    let conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao conectar ao banco de dados"}));
        }
    };

    let row = conn
        .query_opt(
            "SELECT nome, email, senha_hash, perfil::TEXT FROM usuario WHERE email = $1",
            &[&data.email],
        )
        .await;

    let row = match row {
        Ok(Some(r)) => r,
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .json(serde_json::json!({"erro": "Email ou senha inválidos"}));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao consultar usuário"}));
        }
    };

    let nome: String = row.get("nome");
    let email: String = row.get("email");
    let senha_hash: String = row.get("senha_hash");
    let perfil: String = row.get("perfil");

    let parsed_hash = match PasswordHash::new(&senha_hash) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro interno de autenticação"}));
        }
    };

    if Argon2::default()
        .verify_password(data.senha.as_bytes(), &parsed_hash)
        .is_err()
    {
        return HttpResponse::Unauthorized()
            .json(serde_json::json!({"erro": "Email ou senha inválidos"}));
    }

    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: email.clone(),
        perfil: perfil.clone(),
        nome: nome.clone(),
        iat: now,
        exp: now + 8 * 3600, // 8 hours
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao gerar token"}));
        }
    };

    HttpResponse::Ok().json(LoginResponse {
        token,
        usuario: UsuarioInfo {
            nome,
            email,
            perfil,
        },
    })
}

#[post("/api/create-usuario")]
pub async fn create_usuario(
    _admin: AdminAutenticado,
    data: web::Json<CreateUsuarioRequest>,
    pool: web::Data<Pool>,
) -> impl Responder {
    if data.perfil != "admin" && data.perfil != "user" {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"erro": "Perfil deve ser 'admin' ou 'user'"}));
    }

    let salt = SaltString::generate(&mut OsRng);
    let senha_hash = match Argon2::default().hash_password(data.senha.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao processar senha"}));
        }
    };

    let conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao conectar ao banco de dados"}));
        }
    };

    let result = conn
        .execute(
            &format!(
                "INSERT INTO usuario (nome, email, senha_hash, perfil) VALUES ($1, $2, $3, '{}')",
                data.perfil
            ),
            &[&data.nome, &data.email, &senha_hash],
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .json(serde_json::json!({"mensagem": "Usuário criado com sucesso"})),
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                HttpResponse::Conflict()
                    .json(serde_json::json!({"erro": "Email já cadastrado"}))
            } else {
                HttpResponse::InternalServerError()
                    .json(serde_json::json!({"erro": "Erro ao criar usuário"}))
            }
        }
    }
}

#[get("/api/get-usuarios")]
pub async fn get_all_usuarios(
    _admin: AdminAutenticado,
    pool: web::Data<Pool>,
) -> impl Responder {
    let conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"erro": "Erro ao conectar ao banco de dados"}));
        }
    };

    let rows = conn
        .query(
            "SELECT id_usuario, nome, email, perfil::TEXT, data_criacao FROM usuario ORDER BY data_criacao DESC",
            &[],
        )
        .await;

    match rows {
        Ok(rows) => {
            let usuarios: Vec<UsuarioListItem> = rows
                .iter()
                .map(|row| UsuarioListItem {
                    id_usuario: row.get("id_usuario"),
                    nome: row.get("nome"),
                    email: row.get("email"),
                    perfil: row.get("perfil"),
                    data_criacao: row.get("data_criacao"),
                })
                .collect();
            HttpResponse::Ok().json(usuarios)
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"erro": "Erro ao buscar usuários"})),
    }
}

pub async fn seed_admin(pool: &Pool) {
    let conn = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            log::error!("Erro ao conectar ao banco para seed do admin: {}", e);
            return;
        }
    };

    let row = conn
        .query_opt(
            "SELECT id_usuario FROM usuario WHERE perfil = 'admin'::perfil_usuario LIMIT 1",
            &[],
        )
        .await;

    match row {
        Ok(Some(_)) => {
            log::info!("Admin já existe, pulando seed");
        }
        Ok(None) => {
            let salt = SaltString::generate(&mut OsRng);
            let senha_hash = Argon2::default()
                .hash_password(b"admin123", &salt)
                .expect("Erro ao gerar hash da senha do admin")
                .to_string();

            match conn
                .execute(
                    "INSERT INTO usuario (nome, email, senha_hash, perfil) VALUES ($1, $2, $3, 'admin')",
                    &[
                        &"Administrador",
                        &"admin@bgm.com",
                        &senha_hash,
                    ],
                )
                .await
            {
                Ok(_) => {
                    log::warn!("Admin padrão criado: admin@bgm.com / admin123 — ALTERE A SENHA!");
                }
                Err(e) => {
                    log::error!("Erro ao criar admin padrão: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("Erro ao verificar admin existente: {}", e);
        }
    }
}
