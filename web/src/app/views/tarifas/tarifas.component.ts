import { Component } from '@angular/core';
import { MatTabsModule } from '@angular/material/tabs';
import { TarifasListComponent } from "../../components/tarifas-list/tarifas-list.component";
import { TarifasImportComponent } from "../../components/tarifas-import/tarifas-import.component";

@Component({
  selector: 'app-tarifas',
  imports: [TarifasListComponent, MatTabsModule, TarifasImportComponent],
  templateUrl: './tarifas.component.html',
  styleUrl: './tarifas.component.scss'
})
export class TarifasComponent {

}
