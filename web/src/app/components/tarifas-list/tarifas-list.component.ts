import { Component, inject } from '@angular/core';
import { TarifasService } from '../../services/tarifas.service';
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
  selector: 'app-tarifas-list',
  imports: [AgGridAngular],
  templateUrl: './tarifas-list.component.html',
  styleUrl: './tarifas-list.component.scss'
})

export class TarifasListComponent {
  private gridApi!: GridApi;
  public rowData: any[] | null = null;

  public columnDefs: ColDef[] = [
    {
      field: 'id_tarifa',
      headerName: 'ID Tarifa',
      sortable: true,
      filter: true
    },
    {
      field: 'id_tipo_tarifa',
      headerName: 'ID Tipo Tarifa',
      sortable: true,
      filter: true
    },
    {
      field: 'id_pedagio',
      headerName: 'ID Pedágio',
      sortable: true,
      filter: true
    },
    {
      field: 'multiplicador',
      headerName: 'Multiplicador',
      sortable: true,
      filter: true
    },
    {
      field: 'valor',
      headerName: 'Valor',
      sortable: true,
      filter: true
    },
    {
      field: 'data_criacao',
      headerName: 'Data Criação',
      sortable: true,
      filter: true
    },
    {
      field: 'data_atualizacao',
      headerName: 'Data Atualização',
      sortable: true,
      filter: true
    },
    {
      field: 'situacao',
      headerName: 'Situação',
      sortable: true,
      filter: true
    },
    {
      field: 'tipo',
      headerName: 'Tipo',
      sortable: true,
      filter: true
    }
  ];

  tarifasService: TarifasService = inject(TarifasService);

  onGridReady(params: GridReadyEvent) {
    this.gridApi = params.api;
  }

  ngOnInit() {
    this.tarifasService.getTarifas().subscribe((tarifas) => {
      this.rowData = tarifas.body as any[];
    });
  }
}
