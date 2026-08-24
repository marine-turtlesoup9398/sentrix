export class PaymentService {
    public async processPayment(amount: number) {
        return { status: "success", amount };
    }
}
