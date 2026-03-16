# orchestration/airflow/plugins/rs_foundry_client.py
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

    def trigger_bronze_ref(
        self,
        *,
        requested_by: str | None = None,
        orchestration: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return self._post(
            "/jobs/bronze/ref",
            self._build_trigger_payload(
                requested_by=requested_by,
                upstream_run_ids=[],
                orchestration=orchestration,
            ),
        )

    def trigger_bronze_daily(
        self,
        *,
        requested_by: str | None = None,
        orchestration: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return self._post(
            "/jobs/bronze/daily",
            self._build_trigger_payload(
                requested_by=requested_by,
                upstream_run_ids=[],
                orchestration=orchestration,
            ),
        )

    def trigger_silver_ref(
        self,
        bronze_run_id: str,
        *,
        requested_by: str | None = None,
        orchestration: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return self._post(
            "/jobs/silver/ref",
            self._build_trigger_payload(
                requested_by=requested_by,
                upstream_run_ids=[bronze_run_id],
                orchestration=orchestration,
            ),
        )

    def trigger_silver_daily(
        self,
        bronze_run_id: str,
        *,
        requested_by: str | None = None,
        orchestration: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return self._post(
            "/jobs/silver/daily",
            self._build_trigger_payload(
                requested_by=requested_by,
                upstream_run_ids=[bronze_run_id],
                orchestration=orchestration,
            ),
        )

    def trigger_silver_conformed(
        self,
        ref_run_id: str,
        daily_run_id: str,
        *,
        requested_by: str | None = None,
        orchestration: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return self._post(
            "/jobs/silver/conformed",
            self._build_trigger_payload(
                requested_by=requested_by,
                upstream_run_ids=[ref_run_id, daily_run_id],
                orchestration=orchestration,
            ),
        )

    def get_run(self, run_id: str) -> dict[str, Any]:
        return self._get(f"/runs/{run_id}")

    def list_runs(self) -> dict[str, Any]:
        return self._get("/runs")

    def _build_trigger_payload(
        self,
        *,
        requested_by: str | None,
        upstream_run_ids: list[str],
        orchestration: dict[str, Any] | None,
    ) -> dict[str, Any]:
        return {
            "requested_by": requested_by,
            "upstream_run_ids": upstream_run_ids,
            "orchestration": orchestration,
        }

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

        