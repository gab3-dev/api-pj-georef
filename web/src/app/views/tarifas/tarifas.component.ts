import { Component } from '@angular/core';
import { TarifasListComponent } from "../../components/tarifas-list/tarifas-list.component";

@Component({
  selector: 'app-tarifas',
  imports: [TarifasListComponent],
  templateUrl: './tarifas.component.html',
  styleUrl: './tarifas.component.scss'
})
export class TarifasComponent {

}
