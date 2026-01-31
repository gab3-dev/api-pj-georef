import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Component({
    selector: 'app-pedagios-import',
    imports: [],
    templateUrl: './pedagios-import.component.html',
    styleUrl: './pedagios-import.component.scss'
})
export class PedagiosImportComponent {
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

        this.http.post('http://localhost:9999/api/importar-pedagios', formData).subscribe({
            next: (response: any) => {
                // pedagios endpoint returns a proper JSON object
                if (response && response.pedagios_importadas !== undefined) {
                    alert(response.pedagios_importadas + " pedágios importados com sucesso!");
                } else {
                    alert("Importação realizada com sucesso.");
                }
                console.log('File uploaded successfully:', response);
            },
            error: (error: any) => {
                console.log('File upload failed:', error);
                if (error.error && error.error.erro) {
                    alert('Erro: ' + error.error.erro + (error.error.detalhes ? '\n' + error.error.detalhes : ''));
                } else {
                    alert('Falha na importação do arquivo.');
                }
            }
        });
    }
}
