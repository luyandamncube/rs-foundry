# orchestration/airflow/dags/silver_daily_build_dag.py
from __future__ import annotations

import os
import time
from datetime import datetime
from typing import Any

from airflow import DAG
from airflow.providers.standard.operators.python import PythonOperator

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


def trigger_bronze_daily(**context) -> str:
    client = RsFoundryClient.from_env()

    result = client.trigger_bronze_daily(
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    run_id = result["run_id"]

    print(f"bronze_daily trigger response: {result}")
    print(f"captured bronze_daily run_id: {run_id}")

    return run_id


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


def wait_for_bronze_daily(**context) -> dict[str, Any]:
    return wait_for_run("trigger_bronze_daily", **context)


def trigger_silver_daily(**context) -> str:
    client = RsFoundryClient.from_env()

    ti = context["ti"]
    bronze_run = ti.xcom_pull(task_ids="wait_for_bronze_daily")

    if not bronze_run:
        raise RsFoundryClientError("No bronze run payload found in XCom from wait_for_bronze_daily")

    bronze_run_id = bronze_run["run_id"]

    result = client.trigger_silver_daily(
        bronze_run_id,
        requested_by="airflow",
        orchestration=build_orchestration_context(context),
    )
    silver_run_id = result["run_id"]

    print(f"silver_daily trigger response: {result}")
    print(f"upstream bronze_run_id: {bronze_run_id}")
    print(f"captured silver_daily run_id: {silver_run_id}")

    return silver_run_id


def wait_for_silver_daily(**context) -> dict[str, Any]:
    return wait_for_run("trigger_silver_daily", **context)


with DAG(
    dag_id="silver_daily_build_dag",
    description="Build silver daily from an Airflow-triggered bronze daily run.",
    start_date=datetime(2026, 3, 15),
    schedule=None,
    catchup=False,
    tags=["rs-foundry", "phase-9", "silver"],
) as dag:
    trigger_bronze_daily_task = PythonOperator(
        task_id="trigger_bronze_daily",
        python_callable=trigger_bronze_daily,
    )

    wait_for_bronze_daily_task = PythonOperator(
        task_id="wait_for_bronze_daily",
        python_callable=wait_for_bronze_daily,
    )

    trigger_silver_daily_task = PythonOperator(
        task_id="trigger_silver_daily",
        python_callable=trigger_silver_daily,
    )

    wait_for_silver_daily_task = PythonOperator(
        task_id="wait_for_silver_daily",
        python_callable=wait_for_silver_daily,
    )

    (
        trigger_bronze_daily_task
        >> wait_for_bronze_daily_task
        >> trigger_silver_daily_task
        >> wait_for_silver_daily_task
    )