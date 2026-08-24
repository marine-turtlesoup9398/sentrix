import { AuthService } from '../services/auth_service';

export class AuthController {
    private authService: AuthService;

    constructor() {
        this.authService = new AuthService();
    }

    public async login(req: any) {
        return this.authService.authenticate(req.username, req.password);
    }
}

export function handleLoginRequest() {
    const controller = new AuthController();
    return controller.login({ username: "user", password: "pwd" });
}
