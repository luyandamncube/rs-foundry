# Airflow in rs-foundry

This directory contains the local Apache Airflow scaffold for `rs-foundry`.

## Design rule

- Airflow orchestrates
- Rust executes

Airflow DAGs must remain thin. They should call the Rust runner API and should not contain transformation logic, schema logic, contract logic, or output path logic.

## Directory layout

- `dags/` — Airflow DAG definitions
- `plugins/` — shared helper code for DAGs/operators/sensors
- `env/` — local environment configuration
- `logs/` — mounted local Airflow logs
- `config/` — optional Airflow-local config files

## Local startup

1. Build and initialize Airflow
2. Start webserver, scheduler, and triggerer
3. Open the Airflow UI
4. Trigger `ping_runner_dag`

## Expected networking

Airflow reaches the Rust runner over the Docker network using:

`http://runner:8080`

## Current purpose

Phase 7 only proves that Airflow is wired correctly into the local stack.

It does not yet implement full bronze/silver orchestration.