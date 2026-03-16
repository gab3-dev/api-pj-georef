import { Component } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { UsuarioService } from '../../services/usuario.service';

@Component({
  selector: 'app-usuarios-form',
  standalone: true,
  imports: [ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatSelectModule, MatButtonModule],
  templateUrl: './usuarios-form.component.html',
  styleUrl: './usuarios-form.component.scss'
})
export class UsuariosFormComponent {
  mensagem = '';
  erro = '';

  form = new FormGroup({
    nome: new FormControl('', [Validators.required]),
    email: new FormControl('', [Validators.required, Validators.email]),
    senha: new FormControl('', [Validators.required, Validators.minLength(6)]),
    perfil: new FormControl('user', [Validators.required])
  });

  constructor(private usuarioService: UsuarioService) {}

  onSubmit(): void {
    if (this.form.invalid) return;

    this.mensagem = '';
    this.erro = '';

    const { nome, email, senha, perfil } = this.form.value;

    this.usuarioService.createUsuario({
      nome: nome!,
      email: email!,
      senha: senha!,
      perfil: perfil!
    }).subscribe({
      next: (res) => {
        this.mensagem = res.mensagem;
        this.form.reset({ perfil: 'user' });
      },
      error: (err) => {
        this.erro = err.error?.erro || 'Erro ao criar usuário';
      }
    });
  }
}
