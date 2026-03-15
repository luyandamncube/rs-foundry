# orchestration\airflow\plugins\rs_foundry_client.py
import os
from dataclasses import dataclass

import requests


class RsFoundryClientError(RuntimeError):
    pass


@dataclass(frozen=True)
class RsFoundryClient:
    base_url: str

    @classmethod
    def from_env(cls) -> "RsFoundryClient":
        base_url = os.environ.get("RS_FOUNDRY_RUNNER_BASE_URL", "http://runner:8080")
        return cls(base_url=base_url.rstrip("/"))

    def ping_runner(self) -> dict:
        url = f"{self.base_url}/health"
        try:
            response = requests.get(url, timeout=10)
        except requests.RequestException as exc:
            raise RsFoundryClientError(f"Failed to reach runner at {url}: {exc}") from exc

        if not response.ok:
            raise RsFoundryClientError(
                f"Runner health check failed: status={response.status_code}, body={response.text}"
            )

        content_type = response.headers.get("content-type", "")
        if "application/json" in content_type:
            return response.json()

        return {"status": "ok", "raw": response.text}