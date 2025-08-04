import { enableProdMode } from '@angular/core';
import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/settings/app.config';
import { AppComponent } from './app/default/app.component';

if ((window as any).ENABLE_PROD_MODE) {
  enableProdMode();
}

bootstrapApplication(AppComponent, appConfig)
  .catch((err) => console.error(err));