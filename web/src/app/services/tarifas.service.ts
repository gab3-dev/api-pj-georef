import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { FieldInfo } from '../models/field-info';

@Injectable({
  providedIn: 'root'
})
export class TarifasService {

  constructor(private http: HttpClient) { }

  fieldInfoList: FieldInfo[] = [
    {
      id: 1,
      name: 'id_tipo_tarifa',
      label: 'ID Tipo Tarifa',
      type: 'number',
      required: true,
      placeholder: 'Digite o ID do Tipo de Tarifa',
      value: ''
    },
    {
      id: 2,
      name: 'id_pedagio',
      label: 'ID Pedágio',
      type: 'number',
      required: true,
      placeholder: 'Digite o ID do Pedágio',
      value: ''
    },
    {
      id: 3,
      name: 'multiplicador',
      label: 'Multiplicador',
      type: 'number',
      required: true,
      placeholder: 'Digite o Multiplicador',
      value: ''
    },
    {
      id: 4,
      name: 'valor',
      label: 'Valor',
      type: 'number',
      required: true,
      placeholder: 'Digite o Valor',
      value: ''
    },
    {
      id: 5,
      name: 'data_criacao',
      label: 'Data de Criação',
      type: 'date',
      required: true,
      placeholder: 'Selecione a Data de Criação',
      value: ''
    },
    {
      id: 6,
      name: 'data_atualizacao',
      label: 'Data de Atualização',
      type: 'date',
      required: true,
      placeholder: 'Selecione a Data de Atualização',
      value: ''
    },
    {
      id: 7,
      name: 'situacao',
      label: 'Situação',
      type: 'text',
      required: true,
      placeholder: 'Digite a Situação (ex: Ativo)',
      value: ''
    },
    {
      id: 8,
      name: 'tipo',
      label: 'Tipo',
      type: 'text',
      required: true,
      placeholder: 'Digite o Tipo (ex: Normal)',
      value: ''
    }
  ];

  getAllFields(): FieldInfo[] {
    return this.fieldInfoList;
  }

  createTarifa(data: any) {
    // Add dummy values for fields required by the backend struct but not used for creation
    const payload = {
      ...data,
      id_tarifa: 0,
      descricao: '',
      rodagem: '',
      eixos: 0,
      nome: ''
    };

    // Ensure numeric fields are numbers
    payload.id_tipo_tarifa = Number(payload.id_tipo_tarifa);
    payload.id_pedagio = Number(payload.id_pedagio);
    payload.multiplicador = Number(payload.multiplicador);
    payload.valor = Number(payload.valor);

    // Format dates to ISO string if they are not already
    // Assuming the form gives 'yyyy-MM-dd', backend expects 'NaiveDateTime' which fits ISO 8601
    // Ideally we should adhere to what the backend expects. 
    // The backend uses `NaiveDateTime` which usually expects `YYYY-MM-DDTHH:MM:SS`
    // Let's ensure we send a compatible format. 
    if (payload.data_criacao && !payload.data_criacao.includes('T')) {
      payload.data_criacao = `${payload.data_criacao}T00:00:00`;
    }
    if (payload.data_atualizacao && !payload.data_atualizacao.includes('T')) {
      payload.data_atualizacao = `${payload.data_atualizacao}T00:00:00`;
    }

    this.http.post('http://localhost:9999/api/create-tarifa', JSON.stringify(payload), {
      headers: {
        'Content-Type': 'application/json'
      },
      responseType: 'text',
      observe: 'response',
    }).subscribe({
      next: (response) => {
        if (response.status === 200) {
          alert(response.body);
        }
      },
      error: (error) => {
        console.error('Error creating tarifa', error);
        alert('Erro ao criar tarifa');
      }
    });
  }

  getTarifas() {
    // Chama api e formata os dados para objeto
    // API retorna os dados em json
    return this.http.get('http://localhost:9999/api/get-tarifas', {
      responseType: 'json',
      observe: 'response'
    });
  }
}
