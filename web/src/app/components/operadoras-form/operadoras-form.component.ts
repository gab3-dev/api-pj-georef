import { Component, inject } from '@angular/core';
import { FieldInfo } from '../../models/field-info';
import { NgFor, formatDate } from '@angular/common';
import { OperadoraService } from '../../services/operadora.service';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';

@Component({
  selector: 'app-operadoras-form',
  imports: [NgFor, FormsModule, ReactiveFormsModule],
  templateUrl: './operadoras-form.component.html',
  styleUrl: './operadoras-form.component.scss'
})
export class OperadorasFormComponent {
  fieldInfoList: FieldInfo[] = [];
  operadoraService: OperadoraService = inject(OperadoraService);

  operadoraForm = new FormGroup({
    data_alteracao: new FormControl(
      '',
      {
        validators: [Validators.required, Validators.minLength(5), Validators.pattern('^[0-9]{4}-[0-9]{2}-[0-9]{2}$')],
        nonNullable: true,
      }
    ),
    responsavel: new FormControl('', [Validators.required, Validators.minLength(5)]),
    grupo: new FormControl('', [Validators.required, Validators.minLength(5)]),
    codigo_operadora: new FormControl<number>(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    operadora: new FormControl('',),
    razao_social: new FormControl('', [Validators.required, Validators.minLength(5)]),
    cnpj: new FormControl('', [Validators.required, Validators.minLength(5), Validators.pattern('[0-9]{2}\.?[0-9]{3}\.?[0-9]{3}\/?[0-9]{4}\-?[0-9]{2}')]),
    telefone: new FormControl('', [Validators.required, Validators.minLength(5)]),
    email: new FormControl('', [Validators.required, Validators.minLength(5), Validators.email]),
  });

  onSubmit() {
    const data = this.operadoraForm.getRawValue();

    if (data.data_alteracao) {
      data.data_alteracao = formatDate(data.data_alteracao, 'yyyy-MM-dd', 'en-US');
    }

    this.operadoraService.createOperadora(data);
  }

  constructor() {
    this.fieldInfoList = this.operadoraService.getAllFields();
  }
}
