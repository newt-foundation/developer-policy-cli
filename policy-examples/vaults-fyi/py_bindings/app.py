from json import loads, dumps, JSONDecodeError
import wit_world
from wit_world.imports import http
from wit_world.imports.http import HttpRequest, HttpResponse
from datetime import date as _Date

HEADERS = [
    ("Accept", "application/json"),
    ("Content-Type", "application/json"),
    ("User-Agent", "newton-provider/0.1"),
]

URL = "[YOUR LAMBDA URL]/vaults/{network}/{address}"


class WitWorld(wit_world.WitWorld):
    # WIT: run: (input: string) -> result<string, string>
    # In wit_world layout: return str on success; raise Err(str) on error.

    def run(self, input: str) -> str:
        try:
            req = loads(input)

            inquiry_address = req.get("inquiry_address")
            if not inquiry_address:
                return dumps({"error": "inquiry_address_missing"})

            inquiry_network = req.get("inquiry_network")
            if not inquiry_network:
                return dumps({"error": "inquiry_network_missing"})


            url = URL.format(
                address=inquiry_address,
                network=inquiry_network,
            )
            http_req = HttpRequest(url=url, method="GET", headers=[], body=None)
            try:
                fetched: HttpResponse = http.fetch(http_req)
            except NotImplementedError:
                return dumps({"error": "HTTP fetch not implemented by host"})
            except Exception as e:
                return dumps({"error": f"HTTP request failed: {e}"})

            try:
                body= fetched.body.decode("utf-8")
                payload = loads(body)
            except JSONDecodeError as e:
                return dumps(
                    {
                        "message": "unable to fetch user data",
                        "error": str(e),
                        "body": body,
                    }
                )

            return dumps(payload)

        except Exception as e:
            return dumps({"error": str(e)})
