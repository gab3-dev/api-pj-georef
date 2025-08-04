import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PedagiosListComponent } from './pedagios-list.component';

describe('PedagiosListComponent', () => {
  let component: PedagiosListComponent;
  let fixture: ComponentFixture<PedagiosListComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PedagiosListComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PedagiosListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
