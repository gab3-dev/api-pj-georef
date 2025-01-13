import { Component } from '@angular/core';
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
  selector: 'app-tarifas-list',
  imports: [AgGridAngular],
  templateUrl: './tarifas-list.component.html',
  styleUrl: './tarifas-list.component.scss'
})
export class TarifasListComponent {
  public rowData: any[] | null = [
    {
      make: "Fiat",
      model: "500",
      price: 15774,
      electric: false,
      month: "January",
    },
    {
      make: "Nissan",
      model: "Juke",
      price: 20675,
      electric: false,
      month: "March",
    },
    {
      make: "Vauxhall",
      model: "Corsa",
      price: 18460,
      electric: false,
      month: "July",
    },
    {
      make: "Volvo",
      model: "EX30",
      price: 33795,
      electric: true,
      month: "September",
    },
    {
      make: "Mercedes",
      model: "Maybach",
      price: 175720,
      electric: false,
      month: "December",
    },
    {
      make: "Vauxhall",
      model: "Astra",
      price: 25795,
      electric: false,
      month: "April",
    },
    {
      make: "Fiat",
      model: "Panda",
      price: 13724,
      electric: false,
      month: "November",
    },
    {
      make: "Jaguar",
      model: "I-PACE",
      price: 69425,
      electric: true,
      month: "May",
    },
  ];

  public columnDefs: ColDef[] = [
    { field: 'make', flex: 1, minWidth: 100 },
    { field: 'model', flex: 1, minWidth: 100 },
    { field: 'price', flex: 1, minWidth: 100 },
    { field: 'electric', flex: 1, minWidth: 100 },
    { field: 'month', flex: 1, minWidth: 100 },
  ];
}
