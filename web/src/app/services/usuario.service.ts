import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { CreateUsuarioRequest, UsuarioListItem } from '../models/usuario';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class UsuarioService {
  constructor(private http: HttpClient) {}

  createUsuario(data: CreateUsuarioRequest) {
    return this.http.post<{ mensagem: string }>(`${environment.apiUrl}/usuarios`, data);
  }

  getUsuarios() {
    return this.http.get<UsuarioListItem[]>(`${environment.apiUrl}/usuarios`);
  }
}
