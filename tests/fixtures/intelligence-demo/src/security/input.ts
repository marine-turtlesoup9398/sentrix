export function validateInput(val: string) {
    if (!val) throw new Error("Invalid input");
    return val.trim();
}
