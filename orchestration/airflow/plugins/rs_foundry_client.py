# orchestration\airflow\plugins\rs_foundry_client.py
import os
from dataclasses import dataclass
from typing import Any

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

    def ping_runner(self) -> dict[str, Any]:
        return self._get("/health")

    def trigger_bronze_ref(self, payload: dict[str, Any] | None = None) -> dict[str, Any]:
        return self._post("/jobs/bronze/ref", payload or {})

    def trigger_bronze_daily(self, payload: dict[str, Any] | None = None) -> dict[str, Any]:
        return self._post("/jobs/bronze/daily", payload or {})

    def trigger_silver_ref(self, bronze_run_id: str) -> dict[str, Any]:
        return self._post("/jobs/silver/ref", {"bronze_run_id": bronze_run_id})

    def trigger_silver_daily(self, bronze_run_id: str) -> dict[str, Any]:
        return self._post("/jobs/silver/daily", {"bronze_run_id": bronze_run_id})

    def trigger_silver_conformed(self, ref_run_id: str, daily_run_id: str) -> dict[str, Any]:
        return self._post(
            "/jobs/silver/conformed",
            {
                "ref_run_id": ref_run_id,
                "daily_run_id": daily_run_id,
            },
        )

    def get_run(self, run_id: str) -> dict[str, Any]:
        return self._get(f"/runs/{run_id}")

    def list_runs(self) -> dict[str, Any]:
        return self._get("/runs")

    def _get(self, path: str) -> dict[str, Any]:
        url = f"{self.base_url}{path}"
        try:
            response = requests.get(url, timeout=30)
        except requests.RequestException as exc:
            raise RsFoundryClientError(f"GET {url} failed: {exc}") from exc

        return self._handle_response("GET", url, response)

    def _post(self, path: str, payload: dict[str, Any]) -> dict[str, Any]:
        url = f"{self.base_url}{path}"
        try:
            response = requests.post(url, json=payload, timeout=60)
        except requests.RequestException as exc:
            raise RsFoundryClientError(f"POST {url} failed: {exc}") from exc

        return self._handle_response("POST", url, response)

    def _handle_response(self, method: str, url: str, response: requests.Response) -> dict[str, Any]:
        if not response.ok:
            raise RsFoundryClientError(
                f"{method} {url} failed: status={response.status_code}, body={response.text}"
            )

        content_type = response.headers.get("content-type", "")
        if "application/json" not in content_type:
            raise RsFoundryClientError(
                f"{method} {url} returned non-JSON response: {response.text}"
            )

        return response.json()