import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CreateUsuarioRequest, UsuarioListItem } from '../models/usuario';

@Injectable({
  providedIn: 'root'
})
export class UsuarioService {
  constructor(private http: HttpClient) {}

  createUsuario(data: CreateUsuarioRequest) {
    return this.http.post<{ mensagem: string }>('http://localhost:9999/api/create-usuario', data);
  }

  getUsuarios() {
    return this.http.get<UsuarioListItem[]>('http://localhost:9999/api/get-usuarios');
  }
}
