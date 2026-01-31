import { Component, inject } from '@angular/core';
import { FieldInfo } from '../../models/field-info';
import { NgFor, formatDate } from '@angular/common';
import { TarifasService } from '../../services/tarifas.service';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';

@Component({
    selector: 'app-tarifas-form',
    imports: [NgFor, FormsModule, ReactiveFormsModule],
    templateUrl: './tarifas-form.component.html',
    styleUrl: './tarifas-form.component.scss'
})
export class TarifasFormComponent {
    fieldInfoList: FieldInfo[] = [];
    tarifasService: TarifasService = inject(TarifasService);

    tarifasForm = new FormGroup({
        id_tipo_tarifa: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        id_pedagio: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        multiplicador: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        valor: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        data_criacao: new FormControl(
            '',
            {
                validators: [Validators.required],
                nonNullable: true,
            }
        ),
        data_atualizacao: new FormControl(
            '',
            {
                validators: [Validators.required],
                nonNullable: true,
            }
        ),
        situacao: new FormControl('', [Validators.required, Validators.minLength(2)]),
        tipo: new FormControl('', [Validators.required, Validators.minLength(2)]),
    });

    onSubmit() {
        this.tarifasService.createTarifa(this.tarifasForm.value);
    }

    constructor() {
        this.fieldInfoList = this.tarifasService.getAllFields();
    }
}
