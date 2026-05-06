use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UsuarioInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsuarioInfo {
    pub nome: String,
    pub email: String,
    pub perfil: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub perfil: String,
    pub nome: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_seconds: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUsuarioRequest {
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub perfil: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsuarioListItem {
    pub id_usuario: uuid::Uuid,
    pub nome: String,
    pub email: String,
    pub perfil: String,
    pub data_criacao: Option<chrono::NaiveDateTime>,
}
