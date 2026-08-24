import { handleLoginRequest } from '../controllers/auth_controller';

export function routePostLogin(req: any) {
    return handleLoginRequest();
}
