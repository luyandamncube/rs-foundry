# orchestration/airflow/plugins/rs_foundry_airflow.py
from __future__ import annotations

import os
import time
from typing import Any

from rs_foundry_client import RsFoundryClient, RsFoundryClientError


def build_orchestration_context(context: dict[str, Any]) -> dict[str, Any]:
    ti = context["ti"]

    return {
        "orchestrator": "airflow",
        "dag_id": context["dag"].dag_id,
        "dag_run_id": context["run_id"],
        "task_id": context["task"].task_id,
        "try_number": ti.try_number,
    }


def wait_for_run(task_id_to_pull: str, **context) -> dict[str, Any]:
    client = RsFoundryClient.from_env()

    poll_interval = int(os.environ.get("RS_FOUNDRY_POLL_INTERVAL_SECONDS", "5"))
    timeout_seconds = int(os.environ.get("RS_FOUNDRY_RUN_TIMEOUT_SECONDS", "3600"))

    ti = context["ti"]
    run_id = ti.xcom_pull(task_ids=task_id_to_pull)

    if not run_id:
        raise RsFoundryClientError(f"No run_id found in XCom from {task_id_to_pull}")

    start = time.time()

    while True:
        run = client.get_run(run_id)
        status = run["status"]

        print(f"polled run_id={run_id}, status={status}, run={run}")

        if status == "succeeded":
            return run

        if status == "failed":
            error_message = run.get("error_message") or "unknown error"
            raise RsFoundryClientError(
                f"rs-foundry run failed for run_id={run_id}: {error_message}"
            )

        elapsed = time.time() - start
        if elapsed >= timeout_seconds:
            raise RsFoundryClientError(
                f"Timed out waiting for run_id={run_id} after {timeout_seconds} seconds"
            )

        time.sleep(poll_interval)


def get_upstream_run(context: dict[str, Any], task_id: str) -> dict[str, Any]:
    ti = context["ti"]
    run = ti.xcom_pull(task_ids=task_id)

    if not run:
        raise RsFoundryClientError(f"No upstream run payload found in XCom from {task_id}")

    return run


def get_upstream_run_id(context: dict[str, Any], task_id: str) -> str:
    run = get_upstream_run(context, task_id)

    run_id = run.get("run_id")
    if not run_id:
        raise RsFoundryClientError(f"No run_id present in XCom payload from {task_id}")

    return run_id


def trigger_bronze_ref_job(**context) -> str:
    client = RsFoundryClient.from_env()

    result = client.trigger_bronze_ref(
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"bronze_ref trigger response: {result}")
    print(f"captured bronze_ref run_id: {run_id}")

    return run_id


def trigger_bronze_daily_job(**context) -> str:
    client = RsFoundryClient.from_env()

    result = client.trigger_bronze_daily(
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"bronze_daily trigger response: {result}")
    print(f"captured bronze_daily run_id: {run_id}")

    return run_id


def trigger_silver_ref_job(upstream_task_id: str, **context) -> str:
    client = RsFoundryClient.from_env()

    bronze_run_id = get_upstream_run_id(context, upstream_task_id)

    result = client.trigger_silver_ref(
        bronze_run_id,
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"silver_ref trigger response: {result}")
    print(f"upstream bronze_run_id: {bronze_run_id}")
    print(f"captured silver_ref run_id: {run_id}")

    return run_id


def trigger_silver_daily_job(upstream_task_id: str, **context) -> str:
    client = RsFoundryClient.from_env()

    bronze_run_id = get_upstream_run_id(context, upstream_task_id)

    result = client.trigger_silver_daily(
        bronze_run_id,
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"silver_daily trigger response: {result}")
    print(f"upstream bronze_run_id: {bronze_run_id}")
    print(f"captured silver_daily run_id: {run_id}")

    return run_id


def trigger_silver_conformed_job(
    ref_task_id: str,
    daily_task_id: str,
    **context,
) -> str:
    client = RsFoundryClient.from_env()

    ref_run_id = get_upstream_run_id(context, ref_task_id)
    daily_run_id = get_upstream_run_id(context, daily_task_id)

    result = client.trigger_silver_conformed(
        ref_run_id,
        daily_run_id,
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"silver_conformed trigger response: {result}")
    print(f"upstream ref_run_id: {ref_run_id}")
    print(f"upstream daily_run_id: {daily_run_id}")
    print(f"captured silver_conformed run_id: {run_id}")

    return run_id