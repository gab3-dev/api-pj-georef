import { Routes } from '@angular/router';
import { PedagiosComponent } from '../views/pedagios/pedagios.component';
import { OperadorasComponent } from '../views/operadoras/operadoras.component';
import { TarifasComponent } from '../views/tarifas/tarifas.component';
import { HomeComponent } from '../views/home/home.component';
import { LoginComponent } from '../auth/login/login.component';
import { UsuariosComponent } from '../views/usuarios/usuarios.component';
import { authGuard, adminGuard } from '../auth/auth.guard';

export const routes: Routes = [
    {
        path: 'login',
        component: LoginComponent,
        title: 'Login'
    },
    {
        path: '',
        component: HomeComponent,
        title: 'Home page',
        canActivate: [authGuard]
    },
    {
        path: 'operadoras',
        component: OperadorasComponent,
        title: 'Operadoras',
        canActivate: [authGuard]
    },
    {
        path: 'pedagios',
        component: PedagiosComponent,
        title: 'Pedagios',
        canActivate: [authGuard]
    },
    {
        path: 'tarifas',
        component: TarifasComponent,
        title: 'Tarifas',
        canActivate: [authGuard]
    },
    {
        path: 'usuarios',
        component: UsuariosComponent,
        title: 'Usuários',
        canActivate: [authGuard, adminGuard]
    },
    {
        path: '**',
        redirectTo: 'login'
    }
];
