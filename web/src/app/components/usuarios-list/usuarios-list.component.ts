import { Component, OnInit } from '@angular/core';
import { AgGridAngular } from 'ag-grid-angular';
import { ColDef } from 'ag-grid-community';
import { UsuarioService } from '../../services/usuario.service';
import { UsuarioListItem } from '../../models/usuario';

@Component({
  selector: 'app-usuarios-list',
  standalone: true,
  imports: [AgGridAngular],
  templateUrl: './usuarios-list.component.html',
  styleUrl: './usuarios-list.component.scss'
})
export class UsuariosListComponent implements OnInit {
  rowData: UsuarioListItem[] = [];

  colDefs: ColDef[] = [
    { field: 'nome', headerName: 'Nome', sortable: true, filter: true },
    { field: 'email', headerName: 'Email', sortable: true, filter: true },
    { field: 'perfil', headerName: 'Perfil', sortable: true, filter: true },
    { field: 'data_criacao', headerName: 'Data de Criação', sortable: true }
  ];

  constructor(private usuarioService: UsuarioService) {}

  ngOnInit(): void {
    this.usuarioService.getUsuarios().subscribe({
      next: (data) => {
        this.rowData = data;
      },
      error: (err) => {
        console.error('Erro ao buscar usuários:', err);
      }
    });
  }
}
