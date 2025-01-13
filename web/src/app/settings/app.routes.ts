import { Routes } from '@angular/router';
import { PedagiosComponent } from '../views/pedagios/pedagios.component';
import { OperadorasComponent } from '../views/operadoras/operadoras.component';
import { HomeComponent } from '../views/home/home.component';

export const routes: Routes = [
    {
        path: '',
        component: HomeComponent,
        title: 'Home page'
    },
    {
        path: 'pedagios',
        component: PedagiosComponent,
        title: 'Pedagios'
    },
    {
        path: 'operadoras',
        component: OperadorasComponent,
        title: 'Operadoras'
    }
];
