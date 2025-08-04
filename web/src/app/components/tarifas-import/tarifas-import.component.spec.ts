import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TarifasImportComponent } from './tarifas-import.component';

describe('TarifasImportComponent', () => {
  let component: TarifasImportComponent;
  let fixture: ComponentFixture<TarifasImportComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TarifasImportComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(TarifasImportComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
