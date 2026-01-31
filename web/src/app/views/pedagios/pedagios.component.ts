import { Component } from '@angular/core';
import { MatTabsModule } from '@angular/material/tabs';
import { PedagiosFormComponent } from '../../components/pedagios-form/pedagios-form.component';
import { PedagiosListComponent } from '../../components/pedagios-list/pedagios-list.component';
import { PedagiosImportComponent } from '../../components/pedagios-import/pedagios-import.component';

@Component({
  selector: 'app-pedagios',
  standalone: true,
  templateUrl: './pedagios.component.html',
  styleUrl: './pedagios.component.scss',
  imports: [PedagiosListComponent, MatTabsModule, PedagiosFormComponent, PedagiosImportComponent]
})

export class PedagiosComponent {
  title = 'BGM - Pedagios';
}
