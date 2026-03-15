from __future__ import annotations

import os
from datetime import datetime

import requests
from airflow import DAG
from airflow.operators.python import PythonOperator


def ping_runner() -> None:
    base_url = os.environ.get("RS_FOUNDRY_RUNNER_BASE_URL", "http://runner:8080").rstrip("/")
    url = f"{base_url}/health"

    response = requests.get(url, timeout=10)
    response.raise_for_status()

    try:
        payload = response.json()
    except ValueError:
        payload = {"raw": response.text}

    print(f"Runner health response: {payload}")


with DAG(
    dag_id="ping_runner_dag",
    description="Phase 7 validation DAG for rs-foundry Airflow scaffold.",
    start_date=datetime(2026, 3, 15),
    schedule=None,
    catchup=False,
    tags=["rs-foundry", "phase-7", "foundation"],
) as dag:
    ping_runner_task = PythonOperator(
        task_id="ping_runner",
        python_callable=ping_runner,
    )

    ping_runner_task