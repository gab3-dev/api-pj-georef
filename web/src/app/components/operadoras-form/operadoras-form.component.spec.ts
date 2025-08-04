import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OperadorasFormComponent } from './operadoras-form.component';

describe('OperadorasFormComponent', () => {
  let component: OperadorasFormComponent;
  let fixture: ComponentFixture<OperadorasFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OperadorasFormComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(OperadorasFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
