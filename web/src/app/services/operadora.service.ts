import { Injectable } from '@angular/core';
import { FieldInfo } from '../models/field-info';
import { HttpClient } from '@angular/common/http';
import { parseOperadoraJson } from '../utils/handle';

@Injectable({
  providedIn: 'root'
})
export class OperadoraService {
  constructor(private http: HttpClient) { }

  fieldInfoList: FieldInfo[] = [
    {
      id: 1,
      name: 'data_operacao',
      label: 'Data da Operação',
      type: 'date',
      required: true,
      placeholder: 'Digite a Data da operacao',
      value: ''    
    },
    {
      id: 2,
      name: 'responsavel',
      label: 'Responsavel',
      type: 'text',
      required: true,
      placeholder: 'Digite o Responsavel da operadora',
      value: ''
    },
    {
      id: 3,
      name: 'grupo',
      label: 'Grupo',
      type: 'text',
      required: true,
      placeholder: 'Digite o Grupo da operadora',
      value: ''
    },
    {
      id: 4,
      name: 'codigo_operadora',
      label: 'Codigo da Operadora',
      type: 'number',
      required: true,
      placeholder: 'Digite o Codigo da operadora',
      value: ''
    },
    {
      id: 5,
      name: 'operadora',
      label: 'Operadora',
      type: 'text',
      required: true,
      placeholder: 'Digite a Operadora',
      value: ''
    },
    {
      id: 6,
      name: 'razao_social',
      label: 'Razao Social',
      type: 'text',
      required: true,
      placeholder: 'Digite a Razão social da operadora',
      value: ''
    },
    {
      id: 7,
      name: 'cnpj',
      label: 'CNPJ',
      type: 'text',
      required: true,
      placeholder: 'Digite o CNPJ da operadora',
      value: ''
    },
    {
      id: 8,
      name: 'email',
      label: 'Email',
      type: 'email',
      required: true,
      placeholder: 'Digite o Email da operadora',
      value: ''
    },
    {
      id: 9,
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
  
  createOperadora(data: any) {
    data = parseOperadoraJson(data);

    this.http.post('http://ec2-52-67-218-45.sa-east-1.compute.amazonaws.com:9999/create-operadora', data,{
      headers: {
        'Content-Type': 'application/json'
      },
      responseType: 'text',
      observe: 'response',
    },).subscribe(response => {
      if(response.status === 200) {
        alert(response.body);
      }
    });
  }
}
