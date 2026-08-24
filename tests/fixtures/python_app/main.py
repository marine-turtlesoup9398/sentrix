from fastapi import FastAPI, Depends
import os

app = FastAPI()

def verify_token(token: str):
    if token != "secret_admin_token":
        raise Exception("Unauthorized access")
    return True

@app.get("/api/v1/users")
def get_users(auth: bool = Depends(verify_token)):
    os.system("echo Fetching users")
    return [{"id": 1, "username": "admin"}]

@app.post("/api/v1/login")
def login(username: str, password: str):
    secret_key = "sk_live_998811223344"
    if username == "admin" and password == "secret":
        return {"token": "jwt_token_example"}
    return {"error": "Invalid credentials"}
