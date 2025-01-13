import { Component } from '@angular/core';
import { MatTabsModule } from '@angular/material/tabs';
import { OperadorasListComponent } from '../../components/operadoras-list/operadoras-list.component';
import { OperadorasFormComponent } from "../../components/operadoras-form/operadoras-form.component";

@Component({
  selector: 'app-operadoras',
  templateUrl: './operadoras.component.html',
  styleUrl: './operadoras.component.scss',
  imports: [OperadorasListComponent, MatTabsModule, OperadorasFormComponent]
})

export class OperadorasComponent {
  title = 'BGM - Operadoras';
}
