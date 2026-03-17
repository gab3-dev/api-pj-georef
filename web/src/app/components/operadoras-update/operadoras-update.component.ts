import { Component, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatAutocompleteModule } from '@angular/material/autocomplete';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { OperadoraService } from '../../services/operadora.service';

@Component({
  selector: 'app-operadoras-update',
  imports: [ReactiveFormsModule, MatAutocompleteModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatProgressBarModule],
  templateUrl: './operadoras-update.component.html',
  styleUrl: './operadoras-update.component.scss'
})
export class OperadorasUpdateComponent {
  private operadoraService = inject(OperadoraService);

  operadoras: any[] = [];
  filteredOperadoras: any[] = [];
  selectedOperadora: any = null;
  originalValues: Record<string, any> = {};

  searchControl = new FormControl('');
  loading = false;
  successMessage = '';
  errorMessage = '';

  updateForm = new FormGroup({
    responsavel: new FormControl('', [Validators.required]),
    grupo: new FormControl('', [Validators.required]),
    codigo_operadora: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    operadora: new FormControl('', [Validators.required]),
    razao_social: new FormControl('', [Validators.required]),
    cnpj: new FormControl('', [Validators.required]),
    email: new FormControl('', [Validators.required, Validators.email]),
    telefone: new FormControl('', [Validators.required]),
  });

  ngOnInit() {
    this.loadOperadoras();
    this.searchControl.valueChanges.subscribe(value => {
      this.filterOperadoras(value || '');
    });
    this.updateForm.disable();
  }

  loadOperadoras() {
    this.operadoraService.getOperadoras().subscribe({
      next: (response) => {
        this.operadoras = (response.body as any[]) || [];
        this.filteredOperadoras = this.operadoras;
      }
    });
  }

  filterOperadoras(search: string) {
    const term = search.toLowerCase();
    this.filteredOperadoras = this.operadoras.filter(o =>
      o.operadora?.toLowerCase().includes(term) ||
      o.razao_social?.toLowerCase().includes(term) ||
      String(o.codigo_operadora).includes(term)
    );
  }

  displayFn(operadora: any): string {
    return operadora ? `${operadora.codigo_operadora} - ${operadora.operadora}` : '';
  }

  onSelected(operadora: any) {
    this.selectedOperadora = operadora;
    this.successMessage = '';
    this.errorMessage = '';

    const values = {
      responsavel: operadora.responsavel || '',
      grupo: operadora.grupo || '',
      codigo_operadora: operadora.codigo_operadora,
      operadora: operadora.operadora || '',
      razao_social: operadora.razao_social || '',
      cnpj: operadora.cnpj || '',
      email: operadora.email || '',
      telefone: operadora.telefone || '',
    };

    this.originalValues = { ...values };
    this.updateForm.patchValue(values);
    this.updateForm.enable();
  }

  hasChanges(): boolean {
    if (!this.selectedOperadora) return false;
    const current = this.updateForm.getRawValue();
    return Object.keys(this.originalValues).some(
      key => String(current[key as keyof typeof current] ?? '') !== String(this.originalValues[key] ?? '')
    );
  }

  onSubmit() {
    if (!this.selectedOperadora) return;

    if (!this.hasChanges()) {
      alert('Nenhuma alteração detectada. Modifique pelo menos um campo antes de atualizar.');
      return;
    }

    this.loading = true;
    this.successMessage = '';
    this.errorMessage = '';

    const formValues = this.updateForm.getRawValue();
    const data = { ...formValues, data_alteracao: new Date().toISOString().substring(0, 10) };
    this.operadoraService.updateOperadora(this.selectedOperadora.codigo_operadora, data).subscribe({
      next: () => {
        this.loading = false;
        this.successMessage = 'Operadora atualizada com sucesso!';
        this.originalValues = { ...data };
        this.loadOperadoras();
      },
      error: (error) => {
        this.loading = false;
        this.errorMessage = error.error || 'Erro ao atualizar operadora.';
      }
    });
  }
}
