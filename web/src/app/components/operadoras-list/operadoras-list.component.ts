import { Component, inject } from '@angular/core';
import { OperadoraService } from '../../services/operadora.service';
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
  selector: 'app-operadoras-list',
  imports: [AgGridAngular],
  templateUrl: './operadoras-list.component.html',
  styleUrl: './operadoras-list.component.scss'
})
export class OperadorasListComponent {
  public rowData: any[] | null = null;

  public columnDefs: ColDef[] = [
    { field: 'data_alteracao', flex: 1, minWidth: 100 },
    { field: 'responsavel', flex: 1, minWidth: 100 },
    { field: 'grupo', flex: 1, minWidth: 100 },
    { field: 'codigo_operadora', flex: 1, minWidth: 100 },
    { field: 'operadora', flex: 1, minWidth: 100 },
    { field: 'razao_social', flex: 1, minWidth: 100 },
    { field: 'cnpj', flex: 1, minWidth: 100 },
    { field: 'telefone', flex: 1, minWidth: 100 },
    { field: 'email', flex: 1, minWidth: 100 },
  ];

  operadoraService: OperadoraService = inject(OperadoraService);

  constructor() {
    this.rowData = this.operadoraService.getOperadoras();
  }
}
