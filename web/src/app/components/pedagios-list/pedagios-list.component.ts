import { Component, inject } from '@angular/core';
import { PedagioService } from '../../services/pedagio.service';
import { AgGridAngular } from 'ag-grid-angular'; // Angular Data Grid Component
import type { ColDef } from 'ag-grid-community'; // Column Definition Type Interface
import {
  GridApi,
  GridReadyEvent,
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
  private gridApi!: GridApi;
  public rowData: any[] | null = null;

  public columnDefs: ColDef[] = [
    { field: 'codigo', flex: 1, minWidth: 100 },
    { field: 'longitude', flex: 1, minWidth: 100 },
    { field: 'latitude', flex: 1, minWidth: 100 },
    { field: 'codigo_operadora', flex: 1, minWidth: 100 },
    { field: 'nome', flex: 1, minWidth: 100 },
    { field: 'situacao', flex: 1, minWidth: 100 },
    { field: 'rodovia', flex: 1, minWidth: 100 },
    { field: 'km', flex: 1, minWidth: 100 },
    { field: 'cidade', flex: 1, minWidth: 100 },
    { field: 'estado', flex: 1, minWidth: 100 },
    { field: 'codigo_praca', flex: 1, minWidth: 100 },
    { field: 'orientacao', flex: 1, minWidth: 100 },
    { field: 'tipo', flex: 1, minWidth: 100 },
    { field: 'jurisdicao', flex: 1, minWidth: 100 },
    { field: 'tipo_cobranca', flex: 1, minWidth: 100 },
    { field: 'categoria', flex: 1, minWidth: 100 },
    { field: 'data_alteracao', flex: 1, minWidth: 100 },
    { field: 'razao_social', flex: 1, minWidth: 100 },
    { field: 'cnpj', flex: 1, minWidth: 100 },
    { field: 'email', flex: 1, minWidth: 100 },
    { field: 'telefone', flex: 1, minWidth: 100 },
  ];

  pedagioService: PedagioService = inject(PedagioService);

  onGridReady(params: GridReadyEvent) {
    this.gridApi = params.api;
  }

  ngOnInit() {
    this.pedagioService.getPedagios().subscribe({
      next: (operadoras) => {
        this.rowData = operadoras.body as any[];
      }
    });
    this.gridApi.setGridOption("rowData", this.rowData);
  }
}
