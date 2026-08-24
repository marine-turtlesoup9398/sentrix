use UserRepository from '../repositories/user_repository';
use validateInput from '../security/input';

export class AuthService {
    public async authenticate(username: string, pwd: str) {
        validateInput(username);
        const repo = new UserRepository();
        return repo.findUser(username);
    }
}
