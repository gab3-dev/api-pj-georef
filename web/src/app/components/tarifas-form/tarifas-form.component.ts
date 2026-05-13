import { Component, inject } from '@angular/core';
import { FieldInfo } from '../../models/field-info';
import { NgFor } from '@angular/common';
import { TarifasService } from '../../services/tarifas.service';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatAutocompleteModule } from '@angular/material/autocomplete';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { PedagioService } from '../../services/pedagio.service';

@Component({
    selector: 'app-tarifas-form',
    imports: [NgFor, FormsModule, ReactiveFormsModule, MatAutocompleteModule, MatFormFieldModule, MatInputModule],
    templateUrl: './tarifas-form.component.html',
    styleUrl: './tarifas-form.component.scss'
})
export class TarifasFormComponent {
    fieldInfoList: FieldInfo[] = [];
    tarifasService: TarifasService = inject(TarifasService);
    pedagioService: PedagioService = inject(PedagioService);

    pedagios: any[] = [];
    filteredPedagios: any[] = [];
    tiposTarifa: any[] = [];
    filteredTiposTarifa: any[] = [];

    pedagioSearchControl = new FormControl('');
    tipoTarifaSearchControl = new FormControl('');

    tarifasForm = new FormGroup({
        id_tipo_tarifa: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        id_pedagio: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        multiplicador: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        valor: new FormControl<number>(
            null!,
            { validators: [Validators.required], nonNullable: true }
        ),
        data_criacao: new FormControl(
            '',
            {
                validators: [Validators.required],
                nonNullable: true,
            }
        ),
        data_atualizacao: new FormControl(
            '',
            {
                validators: [Validators.required],
                nonNullable: true,
            }
        ),
        situacao: new FormControl('', [Validators.required, Validators.minLength(2)]),
        tipo: new FormControl('', [Validators.required, Validators.minLength(2)]),
    });

    onSubmit() {
        this.tarifasService.createTarifa(this.tarifasForm.value);
    }

    constructor() {
        this.fieldInfoList = this.tarifasService
            .getAllFields()
            .filter(field => !['id_tipo_tarifa', 'id_pedagio'].includes(field.name));
    }

    ngOnInit() {
        this.loadPedagios();
        this.loadTiposTarifa();

        this.pedagioSearchControl.valueChanges.subscribe(value => {
            if (typeof value === 'string') {
                this.tarifasForm.patchValue({ id_pedagio: null! });
                this.filterPedagios(value);
            }
        });

        this.tipoTarifaSearchControl.valueChanges.subscribe(value => {
            if (typeof value === 'string') {
                this.tarifasForm.patchValue({ id_tipo_tarifa: null! });
                this.filterTiposTarifa(value);
            }
        });
    }

    loadPedagios() {
        this.pedagioService.getPedagios().subscribe({
            next: (response) => {
                this.pedagios = ((response.body as any[]) || [])
                    .filter(pedagio => pedagio.id_pedagio !== null && pedagio.id_pedagio !== undefined)
                    .sort((a, b) => Number(a.id_pedagio) - Number(b.id_pedagio));
                this.filteredPedagios = this.pedagios;
            }
        });
    }

    loadTiposTarifa() {
        this.tarifasService.getTiposTarifa().subscribe({
            next: (response) => {
                this.tiposTarifa = ((response.body as any[]) || [])
                    .sort((a, b) => Number(a.id_tipo_tarifa) - Number(b.id_tipo_tarifa));
                this.filteredTiposTarifa = this.tiposTarifa;
            }
        });
    }

    filterPedagios(search: string) {
        const term = search.toLowerCase();
        this.filteredPedagios = this.pedagios.filter(pedagio =>
            String(pedagio.id_pedagio).includes(term) ||
            String(pedagio.codigo_pedagio || '').toLowerCase().includes(term) ||
            String(pedagio.codigo_operadora || '').includes(term) ||
            pedagio.nome?.toLowerCase().includes(term) ||
            pedagio.rodovia?.toLowerCase().includes(term) ||
            pedagio.cidade?.toLowerCase().includes(term)
        );
    }

    filterTiposTarifa(search: string) {
        const term = search.toLowerCase();
        this.filteredTiposTarifa = this.tiposTarifa.filter(tipoTarifa =>
            String(tipoTarifa.id_tipo_tarifa).includes(term) ||
            tipoTarifa.descricao?.toLowerCase().includes(term) ||
            tipoTarifa.rodagem?.toLowerCase().includes(term) ||
            String(tipoTarifa.eixos || '').includes(term)
        );
    }

    displayPedagio(pedagio: any): string {
        if (!pedagio) return '';
        const codigo = pedagio.codigo_pedagio ? ` - Cod. ${pedagio.codigo_pedagio}` : '';
        const rodovia = pedagio.rodovia ? ` - ${pedagio.rodovia}` : '';
        return `${pedagio.id_pedagio} - ${pedagio.nome}${codigo}${rodovia}`;
    }

    displayTipoTarifa(tipoTarifa: any): string {
        if (!tipoTarifa) return '';
        const rodagem = tipoTarifa.rodagem ? ` - ${tipoTarifa.rodagem}` : '';
        const eixos = tipoTarifa.eixos ? ` - ${tipoTarifa.eixos} eixos` : '';
        return `${tipoTarifa.id_tipo_tarifa} - ${tipoTarifa.descricao}${rodagem}${eixos}`;
    }

    onPedagioSelected(pedagio: any) {
        this.tarifasForm.patchValue({ id_pedagio: Number(pedagio.id_pedagio) });
    }

    onTipoTarifaSelected(tipoTarifa: any) {
        this.tarifasForm.patchValue({ id_tipo_tarifa: Number(tipoTarifa.id_tipo_tarifa) });
    }
}
