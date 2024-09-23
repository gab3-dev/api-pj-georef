import { TestBed } from '@angular/core/testing';

import { PracaService } from './praca.service';

describe('PracaService', () => {
  let service: PracaService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PracaService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
