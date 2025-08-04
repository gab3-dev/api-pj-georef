import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TarifasListComponent } from './tarifas-list.component';

describe('TarifasListComponent', () => {
  let component: TarifasListComponent;
  let fixture: ComponentFixture<TarifasListComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TarifasListComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(TarifasListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
