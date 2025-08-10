import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-tarifas-import',
  imports: [],
  templateUrl: './tarifas-import.component.html',
  styleUrl: './tarifas-import.component.scss'
})
export class TarifasImportComponent {
  selectedFile: File | null = null;

  constructor(private http: HttpClient) { }

  onFileSelected(event: Event): void {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      this.selectedFile = input.files[0];
    }
  }

  uploadFile(): void {
    if (!this.selectedFile) {
      return;
    }
    const formData = new FormData();
    formData.append('file', this.selectedFile);

    this.http.post('http://localhost:6969/api/importar-tarifas', formData).subscribe(
      (response: any) => {
        const numero_tarifas_importadas = JSON.parse(response).tarifas_importadas;
        alert(numero_tarifas_importadas + " foram importadas com sucesso!");
        console.log('File uploaded successfully:', response);
      },
      (error: any) => {
        if (error.error.includes("duplicate key value")) {
          alert('Falha na importação do arquivo, coluna já presente no banco de dados: ' + error.error.split("DETAIL: Key ")[1].split(' ')[0]);
        }
        console.log('File upload failed:', error)
      }
    );
  }
}
