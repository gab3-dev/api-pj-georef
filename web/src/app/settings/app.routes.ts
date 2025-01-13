import { Routes } from '@angular/router';
import { PedagiosComponent } from '../views/pedagios/pedagios.component';
import { OperadorasComponent } from '../views/operadoras/operadoras.component';
import { TarifasComponent } from '../views/tarifas/tarifas.component';
import { HomeComponent } from '../views/home/home.component';

export const routes: Routes = [
    {
        path: '',
        component: HomeComponent,
        title: 'Home page'
    },
    {
        path: 'operadoras',
        component: OperadorasComponent,
        title: 'Operadoras'
    },
    {
        path: 'pedagios',
        component: PedagiosComponent,
        title: 'Pedagios'
    },
    {
        path: 'tarifas',
        component: TarifasComponent,
        title: 'Tarifas'
    }
];
