import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PedagiosFormComponent } from './pedagios-form.component';

describe('PedagiosFormComponent', () => {
  let component: PedagiosFormComponent;
  let fixture: ComponentFixture<PedagiosFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PedagiosFormComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PedagiosFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
