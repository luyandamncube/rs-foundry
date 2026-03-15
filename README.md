# rs-foundry

`rs-foundry` is a Rust-first data platform MVP.

It is designed around a simple rule:

- **Rust owns the runtime**
- **notebooks support design-time exploration only**
- **Airflow orchestrates, Rust executes**

The platform provides a clean, production-minded foundation for:

- ingestion
- schema mapping
- transformation
- metadata enrichment
- quality checks
- layered data writes
- run tracking
- API-driven execution
- Airflow-driven orchestration
- OpenAPI-based API exploration
- future worker-based async execution

---

## Core principles

- Rust owns the data plane
- Rust owns the execution logic
- Rust owns the runner API and run registry
- Airflow owns orchestration only
- notebooks are for exploration, validation, and prototyping only
- notebooks are not part of the production execution path
- the platform supports layered outputs:
  - `raw`
  - `bronze`
  - `silver`
  - `gold`

---

## Architecture at a glance

## Control plane

The control plane is responsible for execution management.

It handles:

- job request intake
- request validation
- run creation
- direct execution dispatch
- run status exposure
- API documentation
- future async orchestration

Primary modules:

- `src/api/`
- `src/orchestration/`

Primary entry point:

- `src/bin/runner_api.rs`

---

## Data plane

The data plane is responsible for actual data processing.

It handles:

- reading source inputs
- landing raw payloads
- bronze transformation
- silver standardization
- metadata enrichment
- quality and contract checks
- layered writes

Primary modules:

- `src/io/`
- `src/pipelines/`
- `src/jobs/`
- `src/quality/`

Primary entry points:

- `src/bin/bronze_ref_ingest.rs`
- `src/bin/bronze_daily_ingest.rs`
- `src/bin/silver_ref_build.rs`
- `src/bin/silver_daily_build.rs`
- `src/bin/silver_conformed_build.rs`

---

## Orchestration layer

Airflow is now integrated as the external orchestration layer.

The orchestration rule is:

- **Airflow orchestrates**
- **Rust executes**

Airflow owns:

- DAG scheduling
- dependency ordering
- trigger/wait orchestration
- orchestration visibility
- future retries and backfills

Rust owns:

- ingestion
- transformations
- metadata enrichment
- quality checks
- layered writes
- run creation
- run status updates
- lineage-aware execution semantics

Airflow DAGs remain intentionally thin and call the Rust runner API instead of embedding business logic in Python.

---

## Notebook sidecar

The notebook environment exists only for:

- source inspection
- schema exploration
- transform prototyping
- output validation
- debugging and design-time experimentation

Notebook logic must never be the only place important business logic exists.

Any stable logic should be moved into `src/`.

---

## Layered data model

## Raw

Purpose:
- preserve source fidelity
- support replay/debugging
- keep source-shaped inputs

Location:
- `data/raw/`

## Bronze

Purpose:
- minimally standardized source outputs
- attach ingestion metadata
- apply initial validations

Location:
- `data/bronze/`

## Silver

Purpose:
- cleaned, typed, deduplicated, standardized, and conformed datasets
- stronger quality guarantees
- downstream-ready foundations

Location:
- `data/silver/`

## Gold

Purpose:
- consumer-facing analytics or feature-oriented datasets

Location:
- `data/gold/`

---

## Current MVP status

The current project now includes:

- Rust runner API
- Postgres-backed run registry
- bronze ref job execution
- bronze daily job execution
- silver ref job execution
- silver daily job execution
- silver conformed job execution
- Airflow local orchestration stack
- working trigger/wait DAGs
- OpenAPI generation with Swagger UI

### Working orchestration flows

The following flows are working locally:

- `ping_runner_dag`
- `bronze_ref_ingest_dag`
- `bronze_daily_ingest_dag`
- `silver_ref_build_dag`
- `silver_daily_build_dag`
- `silver_conformed_pipeline_dag`

These prove:

- Airflow can trigger Rust jobs through the runner API
- Airflow can poll the Rust run registry
- Airflow can pass upstream run IDs between steps
- Rust remains the source of truth for execution and run state

---

## OpenAPI / Swagger UI

The runner API is documented with `utoipa` and exposed through Swagger UI.

### Endpoints

- OpenAPI JSON:
  - `/api-docs/openapi.json`
- Swagger UI:
  - `/swagger-ui/`

### Current documented API surface

- `GET /health`
- `GET /ready`
- `POST /jobs/bronze/ref`
- `POST /jobs/bronze/daily`
- `POST /jobs/silver/ref`
- `POST /jobs/silver/daily`
- `POST /jobs/silver/conformed`
- `GET /runs`
- `GET /runs/{id}`

This allows local interactive exploration of the runner API without needing to manually inspect all handlers and models.

---

## Repo layout

```text
rs-foundry/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── docker-compose.yml
├── README.md
├── DIRTREE.md
├── conf/
├── data/
├── docker/
├── docs/
├── notebooks/
├── orchestration/
│   └── airflow/
│       ├── dags/
│       ├── plugins/
│       ├── env/
│       ├── config/
│       └── logs/
├── src/
│   ├── api/
│   ├── config/
│   ├── core/
│   ├── io/
│   ├── jobs/
│   ├── orchestration/
│   ├── pipelines/
│   └── quality/
├── src/bin/
└── tests/
```

## Local development prerequisites

Recommended:

- modern Rust toolchain via `rustup`
- Docker + Docker Compose
- Linux / WSL2 / macOS shell environment

The project uses a Rust toolchain override through `rust-toolchain.toml`.

---

## Local setup

## 1. Check toolchain

```
rustc--version
cargo--version
cargo check
```

## 2. Build + test locally

```
cargo check
cargo test
cargo test--test bronze_tests
cargo test--test silver_tests
cargo test--test quality_tests
cargo test--test api_tests
```

## 3. Run the runner API locally

```
cargo run--bin runner_api
```

Health check:

```
curl http://localhost:8080/health
```

Expected response shape:

```
{"status":"ok","service":"rs-foundry","environment":"base"}
```

---

## Docker Compose usage

## 4. Start the local stack

```
docker compose up--build
```

This starts the core stack:

- postgres
- runner

Optional profile-based services:

- worker
- notebook
- airflow

### Start notebook profile

```
docker compose--profile notebook up--build notebook
```

### Start worker profile

```
docker compose--profile worker up--build worker
```

### Start Airflow profile

```
docker compose--profile airflow up--build
```

This starts:

- `airflow-init`
- `airflow-webserver`
- `airflow-scheduler`
- `airflow-triggerer`
- `airflow-dag-processor`

### Airflow UI

Open:

```
http://localhost:8081
```

### Swagger UI

Open:

```
http://localhost:8080/swagger-ui/
```

---

## Airflow local notes

The Airflow stack is configured for Docker-first local development.

Current local setup includes:

- `LocalExecutor`
- local mounted DAGs and plugins
- runner API access over Docker service networking
- internal Airflow API / execution auth configuration
- local admin-style access for dev-only use

Airflow DAGs call the Rust runner API using the shared helper client in:

- `orchestration/airflow/plugins/rs_foundry_client.py`

The initial DAG pattern is:

- trigger Rust job
- capture returned `run_id`
- poll `GET /runs/{id}`
- succeed/fail based on Rust run status

This pattern is now used across bronze and silver orchestration flows.

---

## Configuration

`rs-foundry` uses a layered, environment-aware configuration model so runtime behavior is explicit and reproducible.

Configuration is loaded in this order:

1. `conf/base/app.toml`
2. `conf/<APP_ENV>/app.toml`
3. environment variables from the shell / container environment

This means:

- `base` contains shared defaults
- `dev` and `prod` contain environment-specific overrides
- environment variables can override file-based settings when needed

### Config directories

- `conf/base/`
- `conf/dev/`
- `conf/prod/`

### Current config files

- `app.toml` — core app/runtime settings
- `sources.toml` — source definitions
- `sinks.toml` — sink/output definitions
- `quality.toml` — quality-rule defaults

### Current runtime settings

The current setup focuses mainly on:

- application name
- host and port
- environment name
- data root path
- config root path
- Postgres connection details

### Important environment variables

- `APP_ENV`
- `APP_NAME`
- `APP_HOST`
- `APP_PORT`
- `DATA_ROOT`
- `CONF_ROOT`
- `POSTGRES_HOST`
- `POSTGRES_PORT`
- `POSTGRES_DB`
- `POSTGRES_USER`
- `POSTGRES_PASSWORD`

For Airflow local orchestration:

- `RS_FOUNDRY_RUNNER_BASE_URL`
- `RS_FOUNDRY_POLL_INTERVAL_SECONDS`
- `RS_FOUNDRY_RUN_TIMEOUT_SECONDS`

---

## Data directories

`rs-foundry` uses an explicit layered local data layout under `data/` so each stage of data maturity is visible and predictable.

### Directory layout

- `data/raw/`
- `data/bronze/`
- `data/silver/`
- `data/gold/`
- `data/checkpoints/`
- `data/runs/`

### Why this layout exists

This structure separates:

- source fidelity
- early-stage operational datasets
- cleaned/conformed datasets
- consumer-facing outputs
- pipeline state and run artifacts

It also aligns the on-disk layout with the architectural model of the platform.

### Rule of thumb

- `raw` preserves what came in
- `bronze` makes it operational
- `silver` makes it trustworthy
- `gold` makes it consumable

### Implementation note

All code should use centralized path helpers instead of manually constructing layer paths in multiple places.

---

## Current runner API flow

The runner API currently supports direct synchronous job execution.

Typical flow:

1. client submits a job request
2. runner creates a `run_id`
3. runner updates run status to `running`
4. Rust job executes
5. run registry is updated to `succeeded` or `failed`
6. client can inspect status through `/runs` or `/runs/{id}`

Airflow builds on top of this by:

1. triggering the runner API
2. capturing the returned `run_id`
3. polling the run registry endpoint
4. propagating upstream run IDs into downstream Rust jobs

---

## Phase roadmap

`rs-foundry` is built in practical phases so the team can establish clean foundations first, then add ingestion, orchestration, and downstream-ready outputs in a controlled way.

### Phase 1 — Foundation

Focus:

- repo scaffold
- Rust module boundaries
- config and path loading
- local Docker Compose stack
- notebook sidecar
- baseline docs

### Phase 2 — Bronze ingestion MVP

Focus:

- raw landing
- bronze reference ingestion
- bronze daily ingestion
- metadata enrichment
- initial schema validation
- basic quality checks

### Phase 3 — Rust control plane MVP

Focus:

- runner API
- job submission endpoints
- run registry
- direct job execution from the API
- run status visibility
- API tests

### Phase 4 — Silver standardization MVP

Focus:

- silver standardization logic
- normalization and type cleanup
- deduplication
- contract checks
- conformed silver dataset build

### Phase 5 — Async orchestration and worker path

Original focus:

- queue abstraction
- worker service
- async run lifecycle
- queued dispatch
- worker/orchestration tests

Current note:

- Airflow has now been introduced as the external orchestration layer for local development
- Rust still owns execution
- future worker-based async execution remains a separate next-step concern

### Phase 6 — Observability, polish, and DS handoff

Focus:

- structured logging
- improved run diagnostics
- data contract documentation
- curated notebooks for profiling and validation
- downstream-facing handoff outputs
- improved orchestration traceability

### Why this sequence

The phases are ordered to avoid premature complexity:

- foundation first
- then ingestion
- then control-plane execution
- then stronger data quality and conformance
- then orchestration maturation
- then polish and handoff

This keeps the MVP grounded in a stable architecture before introducing more complex runtime behavior.