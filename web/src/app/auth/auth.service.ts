import { Injectable, signal, computed } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { LoginRequest, LoginResponse, UsuarioInfo } from '../models/usuario';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private usuarioSignal = signal<UsuarioInfo | null>(this.loadUsuario());
  private tokenSignal = signal<string | null>(this.loadToken());

  usuario = this.usuarioSignal.asReadonly();
  isLoggedIn = computed(() => !!this.tokenSignal());
  isAdmin = computed(() => this.usuarioSignal()?.perfil === 'admin');

  constructor(private http: HttpClient, private router: Router) {}

  private isBrowser(): boolean {
    return typeof window !== 'undefined';
  }

  private loadToken(): string | null {
    if (!this.isBrowser()) return null;
    return localStorage.getItem('token');
  }

  private loadUsuario(): UsuarioInfo | null {
    if (!this.isBrowser()) return null;
    const data = localStorage.getItem('usuario');
    return data ? JSON.parse(data) : null;
  }

  getToken(): string | null {
    return this.tokenSignal();
  }

  login(request: LoginRequest) {
    return this.http.post<LoginResponse>(`${environment.apiUrl}/login`, request);
  }

  handleLoginSuccess(response: LoginResponse): void {
    if (this.isBrowser()) {
      localStorage.setItem('token', response.token);
      localStorage.setItem('usuario', JSON.stringify(response.usuario));
    }
    this.tokenSignal.set(response.token);
    this.usuarioSignal.set(response.usuario);
    this.router.navigate(['/']);
  }

  logout(): void {
    if (this.isBrowser()) {
      localStorage.removeItem('token');
      localStorage.removeItem('usuario');
    }
    this.tokenSignal.set(null);
    this.usuarioSignal.set(null);
    this.router.navigate(['/login']);
  }
}
