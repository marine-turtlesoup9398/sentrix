import { routePostLogin } from '../src/api/auth_api';

export function testAuthFlow() {
    const res = routePostLogin({ username: "admin", password: "secret" });
    return res;
}
