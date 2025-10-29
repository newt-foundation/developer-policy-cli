import os
from typing import Optional
import httpx

from json import JSONDecodeError

from dotenv import load_dotenv
from fastapi import FastAPI
from fastapi import Request
from mangum import Mangum
from slowapi import Limiter, _rate_limit_exceeded_handler
from slowapi.util import get_ipaddr
from slowapi.errors import RateLimitExceeded

load_dotenv()


limiter = Limiter(key_func=get_ipaddr, default_limits=["3/minute"])

app = FastAPI()

app.state.limiter = limiter
app.add_exception_handler(RateLimitExceeded, _rate_limit_exceeded_handler)

VAULTS_API_KEY = os.getenv("VAULTS_API_KEY")


@app.get("/hello")
@limiter.limit("3/minute")
async def hello(request: Request):
    return {"message": "Hello World"}


@app.get("/vaults/{network}/{address}")
@limiter.limit("3/minute")
async def get_vaults_at(request: Request, network: str, address: str):
    return await fetch_vaults_at(network, address)

async def fetch_vaults_at(network: str, address: str) -> dict:
    if not VAULTS_API_KEY:
        raise RuntimeError("VAULTS_API_KEY is not set")

    headers = {"x-api-key": VAULTS_API_KEY}

    async with httpx.AsyncClient() as client:
        response = await client.get(
            f"https://api.vaults.fyi/v2/detailed-vaults/{network}/{address}",
            headers=headers,
        )
        response.raise_for_status()

        try:
            body = response.json()
        except JSONDecodeError:
            raise

        return body


handler = Mangum(app)
