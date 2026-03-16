import { Component, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatAutocompleteModule } from '@angular/material/autocomplete';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { TarifasService } from '../../services/tarifas.service';

@Component({
  selector: 'app-tarifas-update',
  imports: [ReactiveFormsModule, MatAutocompleteModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatProgressBarModule],
  templateUrl: './tarifas-update.component.html',
  styleUrl: './tarifas-update.component.scss'
})
export class TarifasUpdateComponent {
  private tarifasService = inject(TarifasService);

  tarifas: any[] = [];
  filteredTarifas: any[] = [];
  selectedTarifa: any = null;
  originalValues: Record<string, any> = {};

  searchControl = new FormControl('');
  loading = false;
  successMessage = '';
  errorMessage = '';

  updateForm = new FormGroup({
    id_tipo_tarifa: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    id_pedagio: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    multiplicador: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    valor: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    data_criacao: new FormControl('', [Validators.required]),
    situacao: new FormControl('', [Validators.required]),
    tipo: new FormControl('', [Validators.required]),
  });

  ngOnInit() {
    this.loadTarifas();
    this.searchControl.valueChanges.subscribe(value => {
      this.filterTarifas(value || '');
    });
    this.updateForm.disable();
  }

  loadTarifas() {
    this.tarifasService.getTarifas().subscribe({
      next: (response) => {
        this.tarifas = (response.body as any[]) || [];
        this.filteredTarifas = this.tarifas;
      }
    });
  }

  filterTarifas(search: string) {
    const term = search.toLowerCase();
    this.filteredTarifas = this.tarifas.filter(t =>
      t.descricao?.toLowerCase().includes(term) ||
      t.nome?.toLowerCase().includes(term) ||
      t.tipo?.toLowerCase().includes(term) ||
      String(t.id_tarifa).includes(term)
    );
  }

  displayFn(tarifa: any): string {
    return tarifa ? `${tarifa.id_tarifa} - ${tarifa.descricao || tarifa.tipo}` : '';
  }

  private toDateInput(val: string): string {
    if (!val) return '';
    return val.substring(0, 10);
  }

  onSelected(tarifa: any) {
    this.selectedTarifa = tarifa;
    this.successMessage = '';
    this.errorMessage = '';

    const values = {
      id_tipo_tarifa: tarifa.id_tipo_tarifa,
      id_pedagio: tarifa.id_pedagio,
      multiplicador: tarifa.multiplicador,
      valor: tarifa.valor,
      data_criacao: this.toDateInput(tarifa.data_criacao),
      situacao: tarifa.situacao || '',
      tipo: tarifa.tipo || '',
    };

    this.originalValues = { ...values };
    this.updateForm.patchValue(values);
    this.updateForm.enable();
  }

  hasChanges(): boolean {
    if (!this.selectedTarifa) return false;
    const current = this.updateForm.getRawValue();
    return Object.keys(this.originalValues).some(
      key => String(current[key as keyof typeof current] ?? '') !== String(this.originalValues[key] ?? '')
    );
  }

  onSubmit() {
    if (!this.selectedTarifa) return;

    if (!this.hasChanges()) {
      alert('Nenhuma alteração detectada. Modifique pelo menos um campo antes de atualizar.');
      return;
    }

    this.loading = true;
    this.successMessage = '';
    this.errorMessage = '';

    const data = {
      ...this.updateForm.getRawValue(),
      data_atualizacao: new Date().toISOString().substring(0, 10),
      descricao: this.selectedTarifa.descricao || '',
      rodagem: this.selectedTarifa.rodagem || '',
      eixos: this.selectedTarifa.eixos || 0,
      nome: this.selectedTarifa.nome || '',
    };

    this.tarifasService.updateTarifa(this.selectedTarifa.id_tarifa, data).subscribe({
      next: () => {
        this.loading = false;
        this.successMessage = 'Tarifa atualizada com sucesso!';
        this.originalValues = { ...this.updateForm.getRawValue() };
        this.loadTarifas();
      },
      error: (error) => {
        this.loading = false;
        this.errorMessage = error.error || 'Erro ao atualizar tarifa.';
      }
    });
  }
}
