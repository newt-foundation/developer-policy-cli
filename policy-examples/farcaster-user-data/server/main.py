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

EXT_API_KEY = os.getenv("EXT_API_KEY")

@app.get("/hello")
@limiter.limit("3/minute")
async def hello(request: Request):
    return {"message": "Hello World"}


@app.get("/data/{address}")
@limiter.limit("3/minute")
async def data(request: Request, address:str):
    resp = await fetch(address.split(","[0]))
    return resp


async def fetch(addresses: list[str]) -> dict:
    headers = {"x-api-key": EXT_API_KEY}
    params = {"addresses": ",".join(addresses)}

    async with httpx.AsyncClient() as client:
        response = await client.get(
            "https://api.neynar.com/v2/farcaster/user/bulk-by-address/",
            params=params,
            headers=headers,
        )
        response.raise_for_status()

        try:
            ret = response.json()
        except JSONDecodeError:
            raise

        return ret


handler = Mangum(app)
