import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OperadorasListComponent } from './operadoras-list.component';

describe('OperadorasListComponent', () => {
  let component: OperadorasListComponent;
  let fixture: ComponentFixture<OperadorasListComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OperadorasListComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(OperadorasListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
