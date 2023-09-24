import os
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from api.routes import pyme, auth

DIR = os.path.dirname(os.path.abspath(__file__))

app = FastAPI(title="Hotel API")
app.include_router(auth.router)
app.include_router(pyme.router)

app.add_middleware(
    CORSMiddleware,
    allow_origins = [
        "https://hotel-sganis.vercel.app",
        "http://localhost",
        "http://localhost:3000",
        "http://localhost:8000",
        "http://localhost:5173",
        "http://localhost:4173"
    ],
    allow_credentials = True,
    allow_methods = ["*"],
    allow_headers = ["*"],
)


app.mount('/', StaticFiles(directory=f'{DIR}/../client/dist', html=True), name='client')

# test
@app.get("/ping")
def pong():
    return {"ping": "pong!"}

