# orchestration/airflow/dags/silver_ref_build_dag.py
from __future__ import annotations

from datetime import datetime
from functools import partial

from airflow import DAG
from airflow.providers.standard.operators.python import PythonOperator

from rs_foundry_airflow import (
    trigger_bronze_ref_job,
    trigger_silver_ref_job,
    wait_for_run,
)


with DAG(
    dag_id="silver_ref_build_dag",
    description="Build silver ref from an Airflow-triggered bronze ref run.",
    start_date=datetime(2026, 3, 15),
    schedule=None,
    catchup=False,
    tags=["rs-foundry", "phase-9", "silver"],
) as dag:
    trigger_bronze_ref_task = PythonOperator(
        task_id="trigger_bronze_ref",
        python_callable=trigger_bronze_ref_job,
    )

    wait_for_bronze_ref_task = PythonOperator(
        task_id="wait_for_bronze_ref",
        python_callable=partial(wait_for_run, "trigger_bronze_ref"),
    )

    trigger_silver_ref_task = PythonOperator(
        task_id="trigger_silver_ref",
        python_callable=partial(trigger_silver_ref_job, "wait_for_bronze_ref"),
    )

    wait_for_silver_ref_task = PythonOperator(
        task_id="wait_for_silver_ref",
        python_callable=partial(wait_for_run, "trigger_silver_ref"),
    )

    (
        trigger_bronze_ref_task
        >> wait_for_bronze_ref_task
        >> trigger_silver_ref_task
        >> wait_for_silver_ref_task
    )