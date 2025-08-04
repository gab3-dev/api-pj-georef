import { Component, Input } from '@angular/core';
import { FieldInfo } from '../../models/field-info';
import { FormsModule } from '@angular/forms';

@Component({
    selector: 'app-text-field',
    imports: [FormsModule],
    templateUrl: './text-field.component.html',
    styleUrl: './text-field.component.scss'
})

export class TextFieldComponent {
  @Input() fieldInfo!: FieldInfo;
}
