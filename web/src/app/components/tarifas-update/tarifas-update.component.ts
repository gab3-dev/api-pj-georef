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
  pedagios: any[] = [];
  filteredPedagios: any[] = [];
  selectedPedagio: any = null;
  tarifasDoPedagio: any[] = [];
  filteredTarifas: any[] = [];
  selectedTarifa: any = null;
  originalValues: Record<string, any> = {};

  pedagioSearchControl = new FormControl('');
  tarifaSearchControl = new FormControl('');
  loading = false;
  successMessage = '';
  errorMessage = '';

  updateForm = new FormGroup({
    multiplicador: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    valor: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    data_criacao: new FormControl('', [Validators.required]),
    situacao: new FormControl('', [Validators.required]),
    tipo: new FormControl('', [Validators.required]),
  });

  ngOnInit() {
    this.loadTarifas();
    this.pedagioSearchControl.valueChanges.subscribe(value => {
      this.filterPedagios(typeof value === 'string' ? value : this.displayPedagio(value));
    });
    this.tarifaSearchControl.valueChanges.subscribe(value => {
      this.filterTarifas(typeof value === 'string' ? value : this.displayTarifa(value));
    });
    this.updateForm.disable();
  }

  loadTarifas() {
    this.tarifasService.getTarifas().subscribe({
      next: (response) => {
        this.tarifas = (response.body as any[]) || [];
        this.pedagios = this.buildPedagios(this.tarifas);
        this.filteredPedagios = this.pedagios;
        this.setTarifasDoPedagio(this.selectedPedagio);
      }
    });
  }

  private buildPedagios(tarifas: any[]) {
    const pedagios = new Map<number, any>();

    tarifas.forEach(tarifa => {
      if (!pedagios.has(tarifa.id_pedagio)) {
        pedagios.set(tarifa.id_pedagio, {
          id_pedagio: tarifa.id_pedagio,
          nome: tarifa.nome || '',
          codigo_operadora: tarifa.codigo_operadora,
          operadora: tarifa.operadora || '',
        });
      }
    });

    return Array.from(pedagios.values()).sort((a, b) => a.id_pedagio - b.id_pedagio);
  }

  filterPedagios(search: string) {
    const term = search.toLowerCase();
    this.filteredPedagios = this.pedagios.filter(p =>
      p.nome?.toLowerCase().includes(term) ||
      p.operadora?.toLowerCase().includes(term) ||
      String(p.id_pedagio).includes(term) ||
      String(p.codigo_operadora ?? '').includes(term)
    );
  }

  filterTarifas(search: string) {
    const term = search.toLowerCase();
    this.filteredTarifas = this.tarifasDoPedagio.filter(t =>
      t.descricao?.toLowerCase().includes(term) ||
      t.tipo?.toLowerCase().includes(term) ||
      String(t.id_tarifa).includes(term)
    );
  }

  displayPedagio(pedagio: any): string {
    if (!pedagio) return '';
    const operadora = pedagio.operadora ? ` - ${pedagio.operadora}` : '';
    return `${pedagio.id_pedagio} - ${pedagio.nome}${operadora}`;
  }

  displayTarifa(tarifa: any): string {
    return tarifa ? `${tarifa.id_tarifa} - ${tarifa.descricao || tarifa.tipo}` : '';
  }

  onPedagioSelected(pedagio: any) {
    this.selectedPedagio = pedagio;
    this.selectedTarifa = null;
    this.successMessage = '';
    this.errorMessage = '';
    this.updateForm.reset();
    this.updateForm.disable();
    this.setTarifasDoPedagio(pedagio);
    this.tarifaSearchControl.setValue('');
  }

  private setTarifasDoPedagio(pedagio: any) {
    if (!pedagio) {
      this.tarifasDoPedagio = [];
      this.filteredTarifas = [];
      return;
    }

    this.tarifasDoPedagio = this.tarifas.filter(t => t.id_pedagio === pedagio.id_pedagio);
    this.filteredTarifas = this.tarifasDoPedagio;
  }

  private toDateInput(val: string): string {
    if (!val) return '';
    return val.substring(0, 10);
  }

  onTarifaSelected(tarifa: any) {
    this.selectedTarifa = tarifa;
    this.successMessage = '';
    this.errorMessage = '';

    const values = {
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
      id_tipo_tarifa: this.selectedTarifa.id_tipo_tarifa,
      id_pedagio: this.selectedTarifa.id_pedagio,
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
