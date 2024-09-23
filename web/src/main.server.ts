import { bootstrapApplication } from '@angular/platform-browser';
import { AppComponent } from './app/default/app.component';
import { config } from './app/settings/app.config.server';

const bootstrap = () => bootstrapApplication(AppComponent, config);

export default bootstrap;
