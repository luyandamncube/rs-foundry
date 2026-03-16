# orchestration/airflow/dags/silver_daily_build_dag.py
from __future__ import annotations

from datetime import datetime
from functools import partial

from airflow import DAG
from airflow.providers.standard.operators.python import PythonOperator

from rs_foundry_airflow import (
    trigger_bronze_daily_job,
    trigger_silver_daily_job,
    wait_for_run,
)


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
        python_callable=trigger_bronze_daily_job,
    )

    wait_for_bronze_daily_task = PythonOperator(
        task_id="wait_for_bronze_daily",
        python_callable=partial(wait_for_run, "trigger_bronze_daily"),
    )

    trigger_silver_daily_task = PythonOperator(
        task_id="trigger_silver_daily",
        python_callable=partial(trigger_silver_daily_job, "wait_for_bronze_daily"),
    )

    wait_for_silver_daily_task = PythonOperator(
        task_id="wait_for_silver_daily",
        python_callable=partial(wait_for_run, "trigger_silver_daily"),
    )

    (
        trigger_bronze_daily_task
        >> wait_for_bronze_daily_task
        >> trigger_silver_daily_task
        >> wait_for_silver_daily_task
    )