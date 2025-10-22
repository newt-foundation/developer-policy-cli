from json import loads, dumps
import wit_world
from wit_world.imports.http import HttpRequest, HttpResponse
from wit_world.imports import http

URL = "https://sandbox.plaid.com/identity_verification/get"


class WitWorld(wit_world.WitWorld):
    def run(self, input: str) -> str:
        req = loads(input)

        headers = [
            ("Accept", "application/json"),
            ("Content-Type", "application/json"),
            ("User-Agent", "newton-provider/0.1"),
        ]
        body = {
            "client_id": "<your client id here>",
            "secret": "<your secret here>",
            "identity_verification_id": req["verification_id"],
        }

        http_req = HttpRequest(
            url=URL,
            method="POST",
            headers=headers,
            body=body,
        )
        fetched: HttpResponse = http.fetch(http_req)
        payload = loads(fetched.body.decode("utf-8"))

        return dumps(
            {
                "status": payload["status"],
                "steps": payload["steps"],
            }
        )
