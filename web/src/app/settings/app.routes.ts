import { Routes } from '@angular/router';
import { PracasComponent } from '../views/pracas/pracas.component';
import { OperadorasComponent } from '../views/operadoras/operadoras.component';
import { HomeComponent } from '../views/home/home.component';

export const routes: Routes = [
    {
        path: '',
        component: HomeComponent,
        title: 'Home page'
    },
    {
        path: 'pracas',
        component: PracasComponent,
        title: 'Praças'
    },
    {
        path: 'operadoras',
        component: OperadorasComponent,
        title: 'Operadoras'
    }
];
