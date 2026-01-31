import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-operadoras-import',
  imports: [],
  templateUrl: './operadoras-import.component.html',
  styleUrl: './operadoras-import.component.scss'
})
export class OperadorasImportComponent {
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

    this.http.post('http://localhost:9999/api/importar-operadoras', formData).subscribe({
      next: (response: any) => {
        // Backend returns a JSON string inside the body, so we need to parse it
        let resp = response;
        if (typeof response === 'string') {
          try {
            resp = JSON.parse(response);
          } catch (e) {
            console.error("Error parsing JSON response", e);
          }
        }

        if (resp.erro) {
          alert('Falha na importação: ' + resp.erro + (resp.detalhes ? '\n' + resp.detalhes : ''));
        } else if (resp.operadoras_importadas !== undefined) {
          alert(resp.operadoras_importadas + " operadoras importadas com sucesso!");
        } else {
          console.log('Upload success but unknown response format:', resp);
          alert('Upload realizado com sucesso.');
        }
        console.log('File uploaded successfully:', response);
      },
      error: (error: any) => {
        console.log('File upload failed:', error);
        alert('Erro ao enviar arquivo: ' + (error.message || error.statusText));
      }
    });
  }
}
