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
    let result: any[] = [];
    this.http.get('http://localhost:9999/get-tarifas', {
      responseType: 'json',
      observe: 'response'
    }).subscribe(res => {
      console.log(res.body);
      result = res.body as any[];
    });
    console.log(result);
    return result;
  }
}
