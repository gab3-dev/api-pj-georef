import { HttpResponse } from '@angular/common/http';
import { provideHttpClient } from '@angular/common/http';
import { HttpTestingController, provideHttpClientTesting } from '@angular/common/http/testing';
import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CsvImportComponent } from './csv-import.component';

describe('CsvImportComponent', () => {
  let component: CsvImportComponent;
  let fixture: ComponentFixture<CsvImportComponent>;
  let httpMock: HttpTestingController;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CsvImportComponent],
      providers: [
        provideHttpClient(),
        provideHttpClientTesting()
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(CsvImportComponent);
    component = fixture.componentInstance;
    component.apiUrl = '/api/imports/tarifas';
    component.entityLabel = 'Tarifas';
    httpMock = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    httpMock.verify();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should display insert and update counters for tarifa imports', () => {
    spyOn(component.imported, 'emit');
    component.selectedFile = new File(['id_tarifa\n1'], 'tarifas.csv', { type: 'text/csv' });

    component.uploadFile();

    const req = httpMock.expectOne('/api/imports/tarifas');
    expect(req.request.method).toBe('POST');
    req.event(new HttpResponse({
      status: 200,
      body: {
        tarifas_inseridas: 2,
        tarifas_atualizadas: 1,
        tarifas_importadas: 3
      }
    }));

    expect(component.successMessage).toBe('3 Tarifas importados com sucesso! (2 inseridos, 1 atualizados)');
    expect(component.imported.emit).toHaveBeenCalled();
  });

  it('should support legacy import counters returned as JSON strings', () => {
    component.selectedFile = new File(['id_tarifa\n1'], 'tarifas.csv', { type: 'text/csv' });

    component.uploadFile();

    const req = httpMock.expectOne('/api/imports/tarifas');
    req.event(new HttpResponse({
      status: 200,
      body: '{"tarifas_importadas": 4}'
    }));

    expect(component.successMessage).toBe('4 Tarifas importados com sucesso!');
  });
});
