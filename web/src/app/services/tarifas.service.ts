import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class TarifasService {

  constructor(private http: HttpClient) { }

  getTarifas() {
    // Chama api e formata os dados para objeto
    // API retorna os dados em json
    return this.http.get('http://localhost:6969/api/get-tarifas', {
      responseType: 'json',
      observe: 'response'
    });
  }
}
