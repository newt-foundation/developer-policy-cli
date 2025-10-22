# age_module.py
from json import loads, dumps
import wit_world
from wit_world.imports import http
from wit_world.imports.http import HttpRequest, HttpResponse
from datetime import date as _Date

API_BASE = "https://withpersona.com/api/v1/inquiries"


def _age_from_ymd(dob: str, now: str) -> int:
    date_of_birth = _Date.fromisoformat(dob)
    today = _Date.fromisoformat(now)
    years = today.year - date_of_birth.year
    return years - (
        1 if (today.month, today.day) < (date_of_birth.month, date_of_birth.day) else 0
    )


class WitWorld(wit_world.WitWorld):
    # WIT: run: (input: string) -> result<string, string>
    # In wit_world layout: return str on success; raise Err(str) on error.

    def run(self, input: str) -> str:
        try:
            req = loads(input)
            now_ymd = req.get("now_ymd")
            inquiry_id = req.get("inquiry_id")
            if not inquiry_id or not now_ymd:
                return dumps({"error": "inquiry_id_or_now_ymd_missing"})

            url = f"{API_BASE}/{inquiry_id}"
            headers = [
                (
                    "Authorization",
                    "Bearer <your_persona_api_key_here>",
                ),
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

            payload = loads(fetched.body.decode("utf-8"))
            attrs = payload["data"]["attributes"]
            status = attrs.get("status")
            fields = attrs.get("fields")
            behaviors = attrs.get("behaviors")

            if status != "approved":
                return dumps({"error": f"inquiry_not_approved:{status}"})

            birthdate = attrs.get("birthdate")
            age = _age_from_ymd(birthdate, now_ymd)
            if not birthdate:
                return dumps({"error": "birthdate_missing"})

            return dumps(
                {
                    "computed": {
                        "age": age,
                    },
                    "status": status,
                    "bot_score": behaviors.get("bot-score"),
                    "country_code": fields["address-country-code"]["value"],
                    "state": fields["address-subdivision"]["value"],
                }
            )
        except Exception as e:
            return dumps({"error": str(e)})
