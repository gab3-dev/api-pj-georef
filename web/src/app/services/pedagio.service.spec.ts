import { TestBed } from '@angular/core/testing';

import { PedagioService } from './pedagio.service';

describe('PedagioService', () => {
  let service: PedagioService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PedagioService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
