export interface LoginRequest {
  email: string;
  senha: string;
}

export interface LoginResponse {
  token: string;
  usuario: UsuarioInfo;
}

export interface UsuarioInfo {
  nome: string;
  email: string;
  perfil: string;
}

export interface UsuarioListItem {
  id_usuario: string;
  nome: string;
  email: string;
  perfil: string;
  data_criacao: string;
}

export interface CreateUsuarioRequest {
  nome: string;
  email: string;
  senha: string;
  perfil: string;
}
