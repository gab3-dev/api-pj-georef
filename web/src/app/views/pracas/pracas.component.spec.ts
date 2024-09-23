import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PracasComponent } from './pracas.component';

describe('PracasComponent', () => {
  let component: PracasComponent;
  let fixture: ComponentFixture<PracasComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PracasComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PracasComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
