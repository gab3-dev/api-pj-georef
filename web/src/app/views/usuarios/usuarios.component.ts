import { Component } from '@angular/core';
import { MatTabsModule } from '@angular/material/tabs';
import { UsuariosFormComponent } from '../../components/usuarios-form/usuarios-form.component';
import { UsuariosListComponent } from '../../components/usuarios-list/usuarios-list.component';

@Component({
  selector: 'app-usuarios',
  standalone: true,
  imports: [MatTabsModule, UsuariosFormComponent, UsuariosListComponent],
  templateUrl: './usuarios.component.html',
  styleUrl: './usuarios.component.scss'
})
export class UsuariosComponent {}
