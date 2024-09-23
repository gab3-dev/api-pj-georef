import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/settings/app.config';
import { AppComponent } from './app/default/app.component';

bootstrapApplication(AppComponent, appConfig)
  .catch((err) => console.error(err));