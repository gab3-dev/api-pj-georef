import { Component, ViewChild } from '@angular/core';
import { MatTabGroup, MatTabsModule } from '@angular/material/tabs';
import { PedagiosFormComponent } from '../../components/pedagios-form/pedagios-form.component';
import { PedagiosListComponent } from '../../components/pedagios-list/pedagios-list.component';
import { CsvImportComponent } from '../../components/csv-import/csv-import.component';
import { PedagiosUpdateComponent } from '../../components/pedagios-update/pedagios-update.component';
import { AuthService } from '../../auth/auth.service';
import { environment } from '../../../environments/environment';

@Component({
  selector: 'app-pedagios',
  standalone: true,
  templateUrl: './pedagios.component.html',
  styleUrl: './pedagios.component.scss',
  imports: [PedagiosListComponent, MatTabsModule, PedagiosFormComponent, CsvImportComponent, PedagiosUpdateComponent]
})

export class PedagiosComponent {
  title = 'BGM - Pedagios';
  importUrl = `${environment.apiUrl}/imports/pedagios`;

  @ViewChild(MatTabGroup) tabGroup!: MatTabGroup;
  @ViewChild(PedagiosListComponent) listComponent!: PedagiosListComponent;

  constructor(public authService: AuthService) {}

  onImported(): void {
    const listTabIndex = this.authService.isAdmin() ? 1 : 0;
    this.tabGroup.selectedIndex = listTabIndex;
    this.listComponent.reload();
  }
}
