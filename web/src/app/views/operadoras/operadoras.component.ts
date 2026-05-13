import { Component, ViewChild } from '@angular/core';
import { MatTabGroup, MatTabsModule } from '@angular/material/tabs';
import { OperadorasListComponent } from '../../components/operadoras-list/operadoras-list.component';
import { OperadorasFormComponent } from "../../components/operadoras-form/operadoras-form.component";
import { CsvImportComponent } from "../../components/csv-import/csv-import.component";
import { OperadorasUpdateComponent } from "../../components/operadoras-update/operadoras-update.component";
import { AuthService } from '../../auth/auth.service';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-operadoras',
  templateUrl: './operadoras.component.html',
  styleUrl: './operadoras.component.scss',
  imports: [OperadorasListComponent, MatTabsModule, OperadorasFormComponent, CsvImportComponent, OperadorasUpdateComponent]
})

export class OperadorasComponent {
  title = 'BGM - Operadoras';
  importUrl = `${environment.apiUrl}/imports/operadoras`;

  @ViewChild(MatTabGroup) tabGroup!: MatTabGroup;
  @ViewChild(OperadorasListComponent) listComponent!: OperadorasListComponent;

  constructor(public authService: AuthService) {}

  onImported(): void {
    const listTabIndex = this.authService.isAdmin() ? 1 : 0;
    this.tabGroup.selectedIndex = listTabIndex;
    this.listComponent.reload();
  }
}
