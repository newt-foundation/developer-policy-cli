from json import loads, dumps, JSONDecodeError
import wit_world
from wit_world.imports import http
from wit_world.imports.http import HttpRequest, HttpResponse
from datetime import date as _Date


class WitWorld(wit_world.WitWorld):
    # WIT: run: (input: string) -> result<string, string>
    # In wit_world layout: return str on success; raise Err(str) on error.

    def run(self, input: str) -> str:
        try:
            req = loads(input)

            inquiry_address = req.get("inquiry_address")
            if not inquiry_address:
                return dumps({"error": "inquiry_address_missing"})

            url = f"https://o66wu5mr47.execute-api.us-east-2.amazonaws.com/default/data/{inquiry_address}"
            headers = [
                ("Accept", "application/json"),
                ("Content-Type", "application/json"),
                ("User-Agent", "newton-provider/0.1"),
            ]

            http_req = HttpRequest(url=url, method="GET", headers=headers, body=None)
            try:
                fetched: HttpResponse = http.fetch(http_req)
            except NotImplementedError:
                return dumps({"error": "HTTP fetch not implemented by host"})
            except Exception as e:
                return dumps({"error": f"HTTP request failed: {e}"})

            try:
                payload = loads(fetched.body.decode("utf-8"))
                user_data = ([u for k, v in payload.items() for u in v] or [{}])[0]
                score = user_data["score"]
                verified_accounts = user_data["verified_accounts"]
            except (KeyError, TypeError, JSONDecodeError) as e:
                # catch all likely bad formats
                return dumps(
                    {
                        "message": "unable to fetch user data",
                        "error": str(e),
                        "payload": payload,
                    }
                )

            return dumps(
                user_data
            )
        except Exception as e:
            return dumps({"error": str(e)})
