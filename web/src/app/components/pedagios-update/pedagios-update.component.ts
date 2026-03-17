import { Component, inject } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatAutocompleteModule } from '@angular/material/autocomplete';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { PedagioService } from '../../services/pedagio.service';

@Component({
  selector: 'app-pedagios-update',
  imports: [ReactiveFormsModule, MatAutocompleteModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatProgressBarModule],
  templateUrl: './pedagios-update.component.html',
  styleUrl: './pedagios-update.component.scss'
})
export class PedagiosUpdateComponent {
  private pedagioService = inject(PedagioService);

  pedagios: any[] = [];
  filteredPedagios: any[] = [];
  selectedPedagio: any = null;
  originalValues: Record<string, any> = {};

  searchControl = new FormControl('');
  loading = false;
  successMessage = '';
  errorMessage = '';

  updateForm = new FormGroup({
    longitude: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    latitude: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    codigo_operadora: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    nome: new FormControl('', [Validators.required]),
    situacao: new FormControl('', [Validators.required]),
    rodovia: new FormControl('', [Validators.required]),
    km: new FormControl<number>(null!, { validators: [Validators.required], nonNullable: true }),
    sentido: new FormControl('', [Validators.required]),
    cidade: new FormControl('', [Validators.required]),
    estado: new FormControl('', [Validators.required]),
    codigo_pedagio: new FormControl('', [Validators.required]),
    orientacao: new FormControl('', [Validators.required]),
    tipo: new FormControl('', [Validators.required]),
    jurisdicao: new FormControl('', [Validators.required]),
    cobranca_especial: new FormControl('', [Validators.required]),
    categoria: new FormControl('', [Validators.required]),
    razao_social: new FormControl('', [Validators.required]),
    cnpj: new FormControl('', [Validators.required]),
    email: new FormControl('', [Validators.required, Validators.email]),
    telefone: new FormControl('', [Validators.required]),
  });

  ngOnInit() {
    this.loadPedagios();
    this.searchControl.valueChanges.subscribe(value => {
      this.filterPedagios(value || '');
    });
    this.updateForm.disable();
  }

  loadPedagios() {
    this.pedagioService.getPedagios().subscribe({
      next: (response) => {
        this.pedagios = (response.body as any[]) || [];
        this.filteredPedagios = this.pedagios;
      }
    });
  }

  filterPedagios(search: string) {
    const term = search.toLowerCase();
    this.filteredPedagios = this.pedagios.filter(p =>
      p.nome?.toLowerCase().includes(term) ||
      p.rodovia?.toLowerCase().includes(term) ||
      p.cidade?.toLowerCase().includes(term) ||
      String(p.codigo_pedagio).includes(term)
    );
  }

  displayFn(pedagio: any): string {
    return pedagio ? `${pedagio.codigo_pedagio} - ${pedagio.nome}` : '';
  }

  onSelected(pedagio: any) {
    this.selectedPedagio = pedagio;
    this.successMessage = '';
    this.errorMessage = '';

    const values = {
      longitude: pedagio.longitude,
      latitude: pedagio.latitude,
      codigo_operadora: pedagio.codigo_operadora,
      nome: pedagio.nome || '',
      situacao: pedagio.situacao || '',
      rodovia: pedagio.rodovia || '',
      km: pedagio.km,
      sentido: pedagio.sentido || '',
      cidade: pedagio.cidade || '',
      estado: pedagio.estado || '',
      codigo_pedagio: pedagio.codigo_pedagio || '',
      orientacao: pedagio.orientacao || '',
      tipo: pedagio.tipo || '',
      jurisdicao: pedagio.jurisdicao || '',
      cobranca_especial: String(pedagio.cobranca_especial ?? false),
      categoria: pedagio.categoria || '',
      razao_social: pedagio.razao_social || '',
      cnpj: pedagio.cnpj || '',
      email: pedagio.email || '',
      telefone: pedagio.telefone || '',
    };

    this.originalValues = { ...values };
    this.updateForm.patchValue(values);
    this.updateForm.enable();
  }

  hasChanges(): boolean {
    if (!this.selectedPedagio) return false;
    const current = this.updateForm.getRawValue();
    return Object.keys(this.originalValues).some(
      key => String(current[key as keyof typeof current] ?? '') !== String(this.originalValues[key] ?? '')
    );
  }

  onSubmit() {
    if (!this.selectedPedagio) return;

    if (!this.hasChanges()) {
      alert('Nenhuma alteração detectada. Modifique pelo menos um campo antes de atualizar.');
      return;
    }

    this.loading = true;
    this.successMessage = '';
    this.errorMessage = '';

    const data = this.updateForm.getRawValue();
    const payload = {
      ...data,
      cobranca_especial: data.cobranca_especial === 'true',
      data_alteracao: new Date().toISOString().substring(0, 10),
    };

    this.pedagioService.updatePedagio(this.selectedPedagio.codigo_pedagio, payload).subscribe({
      next: () => {
        this.loading = false;
        this.successMessage = 'Pedágio atualizado com sucesso!';
        this.originalValues = { ...data };
        this.loadPedagios();
      },
      error: (error) => {
        this.loading = false;
        this.errorMessage = error.error || 'Erro ao atualizar pedágio.';
      }
    });
  }
}
