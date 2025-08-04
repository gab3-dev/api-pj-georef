import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OperadorasComponent } from './operadoras.component';

describe('OperadorasComponent', () => {
  let component: OperadorasComponent;
  let fixture: ComponentFixture<OperadorasComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OperadorasComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(OperadorasComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
