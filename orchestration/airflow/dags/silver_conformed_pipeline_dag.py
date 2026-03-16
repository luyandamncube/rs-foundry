# orchestration/airflow/dags/silver_conformed_pipeline_dag.py
from __future__ import annotations

from datetime import datetime
from functools import partial

from airflow import DAG
from airflow.providers.standard.operators.python import PythonOperator

from rs_foundry_airflow import (
    trigger_bronze_daily_job,
    trigger_bronze_ref_job,
    trigger_silver_conformed_job,
    trigger_silver_daily_job,
    trigger_silver_ref_job,
    wait_for_run,
)


with DAG(
    dag_id="silver_conformed_pipeline_dag",
    description="Build silver conformed from Airflow-triggered ref and daily silver runs.",
    start_date=datetime(2026, 3, 15),
    schedule=None,
    catchup=False,
    tags=["rs-foundry", "phase-10", "silver", "conformed"],
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

    trigger_silver_conformed_task = PythonOperator(
        task_id="trigger_silver_conformed",
        python_callable=partial(
            trigger_silver_conformed_job,
            "wait_for_bronze_ref",
            "wait_for_bronze_daily",
        ),
    )

    wait_for_silver_conformed_task = PythonOperator(
        task_id="wait_for_silver_conformed",
        python_callable=partial(wait_for_run, "trigger_silver_conformed"),
    )

    (
        trigger_bronze_ref_task
        >> wait_for_bronze_ref_task
        >> trigger_silver_ref_task
        >> wait_for_silver_ref_task
    )

    (
        trigger_bronze_daily_task
        >> wait_for_bronze_daily_task
        >> trigger_silver_daily_task
        >> wait_for_silver_daily_task
    )

    [wait_for_silver_ref_task, wait_for_silver_daily_task] >> trigger_silver_conformed_task
    trigger_silver_conformed_task >> wait_for_silver_conformed_task