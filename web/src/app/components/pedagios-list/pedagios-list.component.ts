import { Component, inject } from '@angular/core';
import { PracaService } from '../../services/praca.service';
import { AgGridAngular } from 'ag-grid-angular'; // Angular Data Grid Component
import type { ColDef } from 'ag-grid-community'; // Column Definition Type Interface
import {
  ModuleRegistry,
  ValueCacheModule,
  AlignedGridsModule,
  ColumnApiModule,
  RowApiModule,
  CellApiModule,
  RenderApiModule,
  ClientSideRowModelModule,
} from "ag-grid-community";
ModuleRegistry.registerModules([ValueCacheModule,
  AlignedGridsModule,
  ColumnApiModule,
  RowApiModule,
  CellApiModule,
  RenderApiModule,
  ClientSideRowModelModule,]);

@Component({
  selector: 'app-pedagios-list',
  imports: [AgGridAngular],
  templateUrl: './pedagios-list.component.html',
  styleUrl: './pedagios-list.component.scss'
})
export class PedagiosListComponent {
  public rowData: any[] | null = null;

  public columnDefs: ColDef[] = [
    { field: 'data_alteracao', flex: 1, minWidth: 100 },
    { field: 'responsavel', flex: 1, minWidth: 100 },
    { field: 'codigo', flex: 1, minWidth: 100 },
    { field: 'nome', flex: 1, minWidth: 100 },
    { field: 'situacao', flex: 1, minWidth: 100 },
    { field: 'codigo_operadora', flex: 1, minWidth: 100 },
    { field: 'razao_social', flex: 1, minWidth: 100 },
    { field: 'cnpj', flex: 1, minWidth: 100 },
    { field: 'telefone', flex: 1, minWidth: 100 },
    { field: 'email', flex: 1, minWidth: 100 },
  ];

  pracaService: PracaService = inject(PracaService);

  constructor() {
    this.rowData = this.pracaService.getPedagios();
  }
}
