use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::{ready, Ready};

use super::models::{Claims, JwtConfig};

pub struct UsuarioAutenticado {
    pub claims: Claims,
}

impl FromRequest for UsuarioAutenticado {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let jwt_config = req.app_data::<web::Data<JwtConfig>>();

        let jwt_config = match jwt_config {
            Some(c) => c,
            None => {
                return ready(Err(actix_web::error::ErrorInternalServerError(
                    "Configuração JWT não encontrada",
                )));
            }
        };

        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(value) => {
                let value_str = match value.to_str() {
                    Ok(v) => v,
                    Err(_) => {
                        return ready(Err(actix_web::error::ErrorUnauthorized(
                            serde_json::json!({"erro": "Token inválido"}),
                        )));
                    }
                };
                if let Some(token) = value_str.strip_prefix("Bearer ") {
                    token.to_string()
                } else {
                    return ready(Err(actix_web::error::ErrorUnauthorized(
                        serde_json::json!({"erro": "Formato de autorização inválido"}),
                    )));
                }
            }
            None => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    serde_json::json!({"erro": "Token de autenticação não fornecido"}),
                )));
            }
        };

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_config.secret.as_bytes()),
            &Validation::default(),
        );

        match token_data {
            Ok(data) => ready(Ok(UsuarioAutenticado {
                claims: data.claims,
            })),
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized(
                serde_json::json!({"erro": "Token inválido ou expirado"}),
            ))),
        }
    }
}

pub struct AdminAutenticado {
    pub claims: Claims,
}

impl FromRequest for AdminAutenticado {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user_result = UsuarioAutenticado::from_request(req, payload);

        match user_result.into_inner() {
            Ok(user) => {
                if user.claims.perfil == "admin" {
                    ready(Ok(AdminAutenticado {
                        claims: user.claims,
                    }))
                } else {
                    ready(Err(actix_web::error::ErrorForbidden(
                        serde_json::json!({"erro": "Acesso restrito a administradores"}),
                    )))
                }
            }
            Err(e) => ready(Err(e)),
        }
    }
}
