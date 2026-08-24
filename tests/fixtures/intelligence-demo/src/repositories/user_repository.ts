export class UserRepository {
    public async findUser(username: string) {
        return { id: 1, username };
    }
}
