import { Component } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { AuthService } from '../auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatCardModule],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {
  erro = '';

  loginForm = new FormGroup({
    email: new FormControl('', [Validators.required, Validators.email]),
    senha: new FormControl('', [Validators.required])
  });

  constructor(private authService: AuthService) {}

  onSubmit(): void {
    if (this.loginForm.invalid) return;

    this.erro = '';
    const { email, senha } = this.loginForm.value;

    this.authService.login({ email: email!, senha: senha! }).subscribe({
      next: (response) => {
        this.authService.handleLoginSuccess(response);
      },
      error: (err) => {
        this.erro = err.error?.erro || 'Erro ao realizar login';
      }
    });
  }
}
