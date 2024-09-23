import { Injectable } from '@angular/core';
import { FieldInfo } from '../models/field-info';
import { HttpClient } from '@angular/common/http';
import { parsePracaJson } from '../utils/handle';

@Injectable({
  providedIn: 'root'
})
export class PracaService {
  constructor(private http: HttpClient) { }
  fieldInfoList: FieldInfo[] = [
    {
      id: 1,
      name: 'codigo',
      label: 'Código',
      type: 'number',
      required: true,
      placeholder: 'Digite o Código',
      value: ''
    },
    {
      id: 2,
      name: 'longitude',
      label: 'Longitude',
      type: 'number',
      required: true,
      placeholder: 'Digite a Longitude',
      value: ''
    },
    {
      id: 3,
      name: 'latitude',
      label: 'Latitude',
      type: 'number',
      required: true,
      placeholder: 'Digite a Latitude',
      value: ''
    },
    {
      id: 4,
      name: 'codigo_operadora',
      label: 'Código da Operadora',
      type: 'number',
      required: true,
      placeholder: 'Digite o Código da Operadora',
      value: ''
    },
    {
      id: 5,
      name: 'nome',
      label: 'Nome',
      type: 'text',
      required: true,
      placeholder: 'Digite o Nome da Praça',
      value: ''
    },
    {
      id: 6,
      name: 'situacao',
      label: 'Situação',
      type: 'text',
      required: true,
      placeholder: 'Digite a Situação',
      value: ''
    },
    {
      id: 7,
      name: 'rodovia',
      label: 'Rodovia',
      type: 'text',
      required: true,
      placeholder: 'Digite a Rodovia',
      value: ''
    },
    {
      id: 8,
      name: 'km',
      label: 'Km',
      type: 'number',
      required: true,
      placeholder: 'Digite o Km',
      value: ''
    },
    {
      id: 10,
      name: 'cidade',
      label: 'Cidade',
      type: 'text',
      required: true,
      placeholder: 'Digite a Cidade',
      value: ''
    },
    {
      id: 11,
      name: 'estado',
      label: 'Estado',
      type: 'text',
      required: true,
      placeholder: 'Digite o Estado',
      value: ''
    },
    {
      id: 12,
      name: 'codigo_praca',
      label: 'Código da Praça',
      type: 'number',
      required: true,
      placeholder: 'Digite o Código da Praça',
      value: ''
    },
    {
      id: 13,
      name: 'orientacao',
      label: 'Orientação',
      type: 'text',
      required: true,
      placeholder: 'Digite a Orientação',
      value: ''
    },
    {
      id: 14,
      name: 'tipo',
      label: 'Tipo',
      type: 'text',
      required: true,
      placeholder: 'Digite o Tipo',
      value: ''
    },
    {
      id: 15,
      name: 'jurisdicao',
      label: 'Jurisdição',
      type: 'text',
      required: true,
      placeholder: 'Digite a Jurisdição',
      value: ''
    },
    {
      id: 16,
      name: 'tipo_cobranca',
      label: 'Tipo de Cobrança',
      type: 'text',
      required: true,
      placeholder: 'Digite o Tipo de Cobrança',
      value: ''
    },
    {
      id: 17,
      name: 'categoria',
      label: 'Categoria',
      type: 'text',
      required: true,
      placeholder: 'Digite a Categoria',
      value: ''
    },
    {
      id: 18,
      name: 'data_alteracao',
      label: 'Data de Alteração',
      type: 'date',
      required: true,
      placeholder: 'Digite a Data de Alteração',
      value: ''
    },
    {
      id: 19,
      name: 'razao_social',
      label: 'Razão Social',
      type: 'text',
      required: true,
      placeholder: 'Digite a Razão Social',
      value: ''
    },
    {
      id: 20,
      name: 'cnpj',
      label: 'CNPJ',
      type: 'text',
      required: true,
      placeholder: 'Digite o CNPJ da operadora',
      value: ''
    },
    {
      id: 21,
      name: 'email',
      label: 'Email',
      type: 'email',
      required: true,
      placeholder: 'Digite o Email da operadora',
      value: ''
    },
    {
      id: 22,
      name: 'telefone',
      label: 'Telefone',
      type: 'text',
      required: true,
      placeholder: 'Digite o Telefone da operadora',
      value: ''
    }
  ];

  getAllFields(): FieldInfo[] {
    return this.fieldInfoList;
  }

  async createPraca(data: any) {
    data = parsePracaJson(data);
    this.http.post('http://ec2-52-67-218-45.sa-east-1.compute.amazonaws.com:9999/create-praca', data, {
      headers: {
        'Content-Type': 'application/json'
      },
      observe: 'response',
    }).subscribe({
      next: response => {
        if (response.status === 200) {
          alert(response.body);
        } else {
          alert(response.body);
        }
      },
      complete: () => {
        console.log('Requisição completa');
        alert('Praça criada com sucesso');
      },
      error: error => {
        if (error)
          console.log(error);
        alert('Erro ao criar a praça');
      }
    });
  }
}
