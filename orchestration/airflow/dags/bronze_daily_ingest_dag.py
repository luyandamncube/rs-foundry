# orchestration\airflow\dags\bronze_daily_ingest_dag.py
from __future__ import annotations

import os
import time
from datetime import datetime
from typing import Any

from airflow import DAG
from airflow.providers.standard.operators.python import PythonOperator

from rs_foundry_client import RsFoundryClient, RsFoundryClientError


def trigger_bronze_daily(**context) -> str:
    client = RsFoundryClient.from_env()

    payload = {
        "triggered_by": "airflow",
        "dag_id": context["dag"].dag_id,
        "dag_run_id": context["run_id"],
        "task_id": context["task"].task_id,
        "logical_date": context["logical_date"].isoformat(),
    }

    result = client.trigger_bronze_daily(payload)
    run_id = result["run_id"]

    print(f"bronze_daily trigger response: {result}")
    print(f"captured run_id: {run_id}")

    return run_id


def wait_for_bronze_daily(**context) -> dict[str, Any]:
    client = RsFoundryClient.from_env()

    poll_interval = int(os.environ.get("RS_FOUNDRY_POLL_INTERVAL_SECONDS", "5"))
    timeout_seconds = int(os.environ.get("RS_FOUNDRY_RUN_TIMEOUT_SECONDS", "3600"))

    ti = context["ti"]
    run_id = ti.xcom_pull(task_ids="trigger_bronze_daily")

    if not run_id:
        raise RsFoundryClientError("No run_id found in XCom from trigger_bronze_daily")

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


with DAG(
    dag_id="bronze_daily_ingest_dag",
    description="Phase 8 DAG that triggers and waits for rs-foundry bronze daily ingest.",
    start_date=datetime(2026, 3, 15),
    schedule=None,
    catchup=False,
    tags=["rs-foundry", "phase-8", "bronze"],
) as dag:
    trigger_bronze_daily_task = PythonOperator(
        task_id="trigger_bronze_daily",
        python_callable=trigger_bronze_daily,
    )

    wait_for_bronze_daily_task = PythonOperator(
        task_id="wait_for_bronze_daily",
        python_callable=wait_for_bronze_daily,
    )

    trigger_bronze_daily_task >> wait_for_bronze_daily_task