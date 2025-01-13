import { Component } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatTabsModule } from '@angular/material/tabs';
import { PedagiosFormComponent } from '../../components/pedagios-form/pedagios-form.component';

@Component({
  selector: 'app-pedagios',
  standalone: true,
  templateUrl: './pedagios.component.html',
  styleUrl: './pedagios.component.scss',
  imports: [FormsModule, ReactiveFormsModule, MatTabsModule, PedagiosFormComponent]
})

export class PedagiosComponent {
  title = 'BGM - Pedagios';
}
