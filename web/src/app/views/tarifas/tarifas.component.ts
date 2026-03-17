import { Component, ViewChild } from '@angular/core';
import { MatTabGroup, MatTabsModule } from '@angular/material/tabs';
import { TarifasListComponent } from "../../components/tarifas-list/tarifas-list.component";
import { TarifasFormComponent } from "../../components/tarifas-form/tarifas-form.component";
import { CsvImportComponent } from "../../components/csv-import/csv-import.component";
import { TarifasUpdateComponent } from "../../components/tarifas-update/tarifas-update.component";
import { AuthService } from '../../auth/auth.service';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-tarifas',
  imports: [TarifasListComponent, MatTabsModule, TarifasFormComponent, CsvImportComponent, TarifasUpdateComponent],
  templateUrl: './tarifas.component.html',
  styleUrl: './tarifas.component.scss'
})
export class TarifasComponent {
  importUrl = `${environment.apiUrl}/importar-tarifas`;
  @ViewChild(MatTabGroup) tabGroup!: MatTabGroup;
  @ViewChild(TarifasListComponent) listComponent!: TarifasListComponent;

  constructor(public authService: AuthService) {}

  onImported(): void {
    const listTabIndex = this.authService.isAdmin() ? 1 : 0;
    this.tabGroup.selectedIndex = listTabIndex;
    this.listComponent.reload();
  }
}
