import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { NavbarComponent } from "../components/navbar/navbar.component";
import { AuthService } from '../auth/auth.service';

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrl: './app.component.scss',
    imports: [RouterModule, NavbarComponent]
})

export class AppComponent {
  title = 'BGM';
  constructor(public authService: AuthService) {}
}
