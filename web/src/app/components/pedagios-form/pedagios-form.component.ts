import { Component, inject } from '@angular/core';
import { FieldInfo } from '../../models/field-info';
import { NgFor, formatDate } from '@angular/common';
import { PedagioService } from '../../services/pedagio.service';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';

@Component({
  selector: 'app-pedagios-form',
  imports: [NgFor, FormsModule, ReactiveFormsModule],
  templateUrl: './pedagios-form.component.html',
  styleUrl: './pedagios-form.component.scss'
})
export class PedagiosFormComponent {
  fieldInfoList: FieldInfo[] = [];
  PedagioService: PedagioService = inject(PedagioService);

  pracaForm = new FormGroup({
    codigo: new FormControl<number>(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    longitude: new FormControl(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    latitude: new FormControl(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    codigo_operadora: new FormControl<number>(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    nome: new FormControl('', [Validators.required, Validators.minLength(5)]),
    situacao: new FormControl('', [Validators.required, Validators.minLength(5)]),
    rodovia: new FormControl('', [Validators.required, Validators.minLength(5)]),
    km: new FormControl<number>(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    sentido: new FormControl('', [Validators.required, Validators.minLength(3)]),
    cidade: new FormControl('', [Validators.required, Validators.minLength(5)]),
    estado: new FormControl('', [Validators.required, Validators.minLength(5)]),
    codigo_praca: new FormControl<number>(
      null!,
      { validators: [Validators.required], nonNullable: true }
    ),
    orientacao: new FormControl('', [Validators.required, Validators.minLength(5)]),
    tipo: new FormControl('', [Validators.required, Validators.minLength(5)]),
    jurisdicao: new FormControl('', [Validators.required, Validators.minLength(5)]),
    tipo_cobranca: new FormControl('', [Validators.required, Validators.minLength(5)]),
    categoria: new FormControl('', [Validators.required, Validators.minLength(5)]),
    data_alteracao: new FormControl(
      '',
      {
        validators: [Validators.required, Validators.minLength(5), Validators.pattern('^[0-9]{4}-[0-9]{2}-[0-9]{2}$')],
        nonNullable: true,
      }
    ),
    razao_social: new FormControl('', [Validators.required, Validators.minLength(5)]),
    cnpj: new FormControl('', [Validators.required, Validators.minLength(5), Validators.pattern('[0-9]{2}\.?[0-9]{3}\.?[0-9]{3}\/?[0-9]{4}\-?[0-9]{2}')]),
    telefone: new FormControl('', [Validators.required, Validators.minLength(5)]),
    email: new FormControl('', [Validators.required, Validators.minLength(5), Validators.email]),
    cobranca_especial: new FormControl('', [Validators.required, Validators.minLength(4)]),
  });

  onSubmit() {
    if (this.pracaForm.value.data_alteracao) {
      this.pracaForm.value.data_alteracao = formatDate(this.pracaForm.value.data_alteracao, 'yyyy-MM-dd', 'en-US');
    }
    this.PedagioService.createPraca(this.pracaForm.value);
  }

  constructor() {
    this.fieldInfoList = this.PedagioService.getAllFields();
  }
}
