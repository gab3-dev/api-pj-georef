import { Component, ElementRef, EventEmitter, Input, Output, ViewChild } from '@angular/core';
import { HttpClient, HttpEventType } from '@angular/common/http';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-csv-import',
  imports: [MatProgressBarModule, MatButtonModule, MatIconModule],
  templateUrl: './csv-import.component.html',
  styleUrl: './csv-import.component.scss'
})
export class CsvImportComponent {
  @Input() apiUrl = '';
  @Input() entityLabel = '';
  @Output() imported = new EventEmitter<void>();

  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;

  selectedFile: File | null = null;
  uploading = false;
  successMessage = '';
  errorMessage = '';
  dragOver = false;

  constructor(private http: HttpClient) {}

  onFileSelected(event: Event): void {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      this.selectFile(input.files[0]);
    }
  }

  onDragOver(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.dragOver = true;
  }

  onDragLeave(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.dragOver = false;
  }

  onDrop(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.dragOver = false;
    if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
      this.selectFile(event.dataTransfer.files[0]);
    }
  }

  private selectFile(file: File): void {
    this.selectedFile = file;
    this.successMessage = '';
    this.errorMessage = '';
  }

  uploadFile(): void {
    if (!this.selectedFile || this.uploading) {
      return;
    }

    this.uploading = true;
    this.successMessage = '';
    this.errorMessage = '';

    const formData = new FormData();
    formData.append('file', this.selectedFile);

    this.http.post(this.apiUrl, formData, {
      reportProgress: true,
      observe: 'events'
    }).subscribe({
      next: (event) => {
        if (event.type === HttpEventType.Response) {
          this.uploading = false;
          this.successMessage = this.buildSuccessMessage(event.body);

          this.resetFileInput();
          this.imported.emit();
        }
      },
      error: (error) => {
        this.uploading = false;
        if (error.error?.erro) {
          this.errorMessage = error.error.erro + (error.error.detalhes ? '\n' + error.error.detalhes : '');
        } else if (typeof error.error === 'string') {
          this.errorMessage = error.error;
        } else {
          this.errorMessage = 'Erro ao enviar arquivo: ' + (error.message || error.statusText);
        }
      }
    });
  }

  private resetFileInput(): void {
    this.selectedFile = null;
    if (this.fileInput) {
      this.fileInput.nativeElement.value = '';
    }
  }

  private buildSuccessMessage(responseBody: unknown): string {
    const body = this.parseResponseBody(responseBody);
    const inserted = this.readNumber(body, 'tarifas_inseridas');
    const updated = this.readNumber(body, 'tarifas_atualizadas');

    if (inserted !== undefined && updated !== undefined) {
      const total = this.readNumber(body, 'tarifas_importadas') ?? inserted + updated;
      return `${total} ${this.entityLabel} importados com sucesso! (${inserted} inseridos, ${updated} atualizados)`;
    }

    const count = this.findImportedCount(body);
    if (count !== undefined) {
      return `${count} ${this.entityLabel} importados com sucesso!`;
    }

    return `${this.entityLabel} importados com sucesso!`;
  }

  private parseResponseBody(responseBody: unknown): Record<string, unknown> | null {
    if (!responseBody) {
      return null;
    }

    if (typeof responseBody === 'string') {
      try {
        const parsed = JSON.parse(responseBody);
        return this.isObject(parsed) ? parsed : null;
      } catch {
        return null;
      }
    }

    return this.isObject(responseBody) ? responseBody : null;
  }

  private findImportedCount(body: Record<string, unknown> | null): number | undefined {
    if (!body) {
      return undefined;
    }

    for (const key of Object.keys(body)) {
      if (key.includes('importad')) {
        return this.toNumber(body[key]);
      }
    }

    return undefined;
  }

  private readNumber(body: Record<string, unknown> | null, key: string): number | undefined {
    return body ? this.toNumber(body[key]) : undefined;
  }

  private toNumber(value: unknown): number | undefined {
    return typeof value === 'number' ? value : undefined;
  }

  private isObject(value: unknown): value is Record<string, unknown> {
    return typeof value === 'object' && value !== null && !Array.isArray(value);
  }
}
