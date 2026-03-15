    # PHASE 7 — Airflow Foundation

## Objective

Introduce **Apache Airflow** into `rs-foundry` as the **external orchestration layer** without replacing the existing Rust execution platform.

This phase establishes the **minimum production-minded foundation** required for Airflow to:

- run inside the current Docker Compose stack
- connect to the existing `runner` service
- use Postgres for Airflow metadata
- support thin DAGs that trigger Rust jobs through the runner API
- preserve `rs-foundry` run identity and lineage

This phase does **not** implement full multi-step bronze → silver → conformed orchestration yet.  
It focuses on creating the orchestration scaffold correctly.

---

## Phase outcome

By the end of Phase 7, the project should support:

- Airflow running locally via Docker Compose
- Airflow web UI available locally
- Airflow scheduler and triggerer running correctly
- Airflow metadata persisted in Postgres
- a thin test DAG that can reach the Rust runner service
- a documented integration path for all future DAGs

This phase is the **platform foundation** for later Airflow-driven orchestration work.

---

## Architectural decision for this phase

## Airflow triggers the Rust runner API

For `rs-foundry`, Airflow will integrate with the platform by calling the **Rust runner API**, not by directly invoking Rust binaries as the default path.

### Why this is the chosen integration pattern

Because `rs-foundry` already has the correct control-plane seam:

- a Rust runner API
- a Postgres-backed run registry
- existing job entrypoints
- existing run identity (`run_id`)
- existing layered execution model

Using the runner API preserves:

- one execution ingress
- one place for run creation
- one place for job validation
- one place for lineage-aware handoff
- one place for future auth/policy enforcement
- one place for execution metadata and logging linkage

If Airflow directly invokes Rust containers or CLI bins by default, then orchestration logic becomes fragmented across:

- Airflow DAG code
- shell/container commands
- CLI parameter parsing
- duplicated metadata propagation

That would weaken the current Rust control plane instead of building on top of it.

### Rule for this phase

**Airflow orchestrates. Rust executes.**

Airflow owns:

- schedule framework
- orchestration visibility
- DAG/task dependency modeling
- retries
- backfills
- trigger/wait logic

Rust owns:

- ingestion
- transformation
- schema mapping
- metadata enrichment
- contracts
- quality checks
- output writes
- run registry updates
- lineage-aware output logic

---

## Scope

## In scope

This phase includes:

- adding Airflow services to Docker Compose
- creating a dedicated Airflow Docker image
- wiring Airflow to Postgres
- wiring Airflow to the Rust runner service over the Docker network
- creating Airflow repo structure (`dags`, `plugins`, `env`, `logs`)
- adding a minimal validation DAG
- documenting the service topology and integration model

## Out of scope

This phase does **not** include:

- full bronze DAG implementation
- full silver DAG implementation
- conformed fan-in workflows
- advanced backfill strategy
- production deployment beyond local Docker-first setup
- replacing internal Rust execution code
- moving transformation logic into Python
- introducing Celery/KubernetesExecutor
- deleting Rust scheduler/dispatcher components yet

---

## Target architecture after this phase

After Phase 7, the local architecture should look like this:

```text
Airflow Scheduler / Webserver / Triggerer
                |
                v
        Thin DAGs / helper client
                |
                v
       Rust Runner API (runner)
                |
                v
      Rust jobs / pipelines / contracts
                |
                v
 raw / bronze / silver / gold outputs
                |
                v
     Postgres run registry + metadata
```

Airflow should be able to:

- resolve the `runner` service by Docker service name
- send HTTP requests to the runner API
- later pass orchestration metadata into job triggers
- later poll for completion through `GET /runs/:id`

---

## Service design

## Airflow services to add

The following Airflow services should be added to the current Compose stack:

- `airflow-init`
- `airflow-webserver`
- `airflow-scheduler`
- `airflow-triggerer`

Optional later:

- `airflow-cli`

For this phase, these four are sufficient.

### Why these services

### `airflow-init`

Used once to initialize the Airflow metadata database and bootstrap the first admin user.

### `airflow-webserver`

Provides the Airflow UI for local workflow visibility and manual trigger/testing.

### `airflow-scheduler`

Parses DAGs, schedules runs, and manages task orchestration.

### `airflow-triggerer`

Supports deferred execution patterns and is useful even early when sensors/deferrable operators are introduced later.

---

## Executor choice

For Phase 7, use:

**`LocalExecutor`**

### Why `LocalExecutor`

It is the best local-first compromise because it:

- is closer to a realistic orchestration runtime than `SequentialExecutor`
- avoids the complexity of Celery for now
- keeps local setup small enough for Docker Compose
- is sufficient for early DAG development and testing

Do **not** introduce Celery, Redis, or a dedicated Airflow worker layer in this phase.

---

## Postgres design

## Recommendation

Reuse the existing `postgres` service and create a separate Airflow metadata database inside it.

Recommended DB separation:

- `rs_foundry`
- `airflow`

### Why this is the right call for now

It keeps the local stack smaller and simpler:

- fewer containers
- simpler networking
- easier debugging
- less operational overhead

Airflow metadata and `rs-foundry` execution metadata remain logically separate by database, even if they share the same container in local development.

---

## Docker Compose design

## Required service-level behavior

### `postgres`

Must be available before `airflow-init`, `airflow-webserver`, `airflow-scheduler`, and `airflow-triggerer` begin normal operation.

### `runner`

Must remain reachable from Airflow by service name, for example:

```
http://runner:8080
```

### Airflow containers

Must all share:

- the same Airflow image
- the same DAG mount
- the same plugins mount
- the same log mount
- the same environment configuration

---

## Required mounts

The following mounts should be introduced:

- `./orchestration/airflow/dags:/opt/airflow/dags`
- `./orchestration/airflow/plugins:/opt/airflow/plugins`
- `./orchestration/airflow/logs:/opt/airflow/logs`
- `./orchestration/airflow/config:/opt/airflow/config`

This allows:

- DAG editing from the repo
- reusable helper code for thin DAGs
- persisted Airflow logs locally
- environment-specific local Airflow configuration

---

## Required environment variables

At minimum, Airflow should be configured with values equivalent to:

```
AIRFLOW__CORE__EXECUTOR=LocalExecutor
AIRFLOW__CORE__LOAD_EXAMPLES=False
AIRFLOW__DATABASE__SQL_ALCHEMY_CONN=postgresql+psycopg2://postgres:postgres@postgres:5432/airflow
AIRFLOW__WEBSERVER__EXPOSE_CONFIG=False
RS_FOUNDRY_RUNNER_BASE_URL=http://runner:8080
RS_FOUNDRY_POLL_INTERVAL_SECONDS=5
RS_FOUNDRY_RUN_TIMEOUT_SECONDS=3600
```

These values may live in:

```
orchestration/airflow/env/airflow.env
```

---

## Recommended repo additions

This phase should add the following structure:

```
rs-foundry/
├── docker/
│   └── airflow.Dockerfile
├── orchestration/
│   └── airflow/
│       ├── README.md
│       ├── dags/
│       │   └── ping_runner_dag.py
│       ├── plugins/
│       │   └── rs_foundry_client.py
│       ├── config/
│       │   └── airflow_local_settings.py
│       ├── env/
│       │   └── airflow.env
│       └── logs/
│           └── .gitkeep
├── docs/
│   ├── airflow.md
│   └── phases/
│       └── PHASE_7_AIRFLOW_FOUNDATION.md
```

---

## File responsibilities

## `docker/airflow.Dockerfile`

Purpose:

- build a pinned Airflow image for local development
- install only the Python dependencies needed for thin DAG orchestration
- keep DAG execution predictable across machines

This image should include:

- Apache Airflow
- Postgres driver
- lightweight HTTP client support
- any minimal helper dependency needed by the thin DAG layer

It should **not** include business logic.

---

## `orchestration/airflow/dags/`

Purpose:

- hold Airflow DAG definitions only
- remain thin
- avoid transformation code
- avoid data contracts
- avoid file path logic
- avoid embedded business rules

In this phase, this folder only needs a minimal validation DAG.

---

## `orchestration/airflow/plugins/rs_foundry_client.py`

Purpose:

- provide a tiny helper client for DAG code
- centralize HTTP requests to the Rust runner API
- avoid repeating request boilerplate in every DAG

In this phase, it can be very small, such as:

- `ping_runner()`
- later: `trigger_job()`
- later: `get_run()`

---

## `orchestration/airflow/README.md`

Purpose:

- explain the local orchestration layout
- explain where DAGs live
- explain how Airflow connects to the runner
- explain how to bring the stack up

---

## `docs/airflow.md`

Purpose:

- document the architectural role of Airflow in `rs-foundry`
- explain why Airflow calls the runner API
- explain what remains in Rust
- explain what future DAGs should and should not do

---

## First validation DAG for this phase

The first DAG in this phase should **not** be a real bronze workflow yet.

It should be a tiny scaffold DAG such as:

```
ping_runner_dag
```

### What it should do

- execute manually or on a harmless schedule
- call a simple runner endpoint such as `/health` if available
- prove that:
    - DAG parsing works
    - scheduler sees the DAG
    - Airflow can reach `runner`
    - logs are visible in the UI

### Why start here

Because this phase is about **platform wiring**, not data workflow complexity.

Before building real orchestration DAGs, we must first prove:

- Airflow starts cleanly
- mounts work
- Airflow can import local helper code
- the Compose network is correct
- the runner is reachable

That is the lowest-risk, highest-signal first check.

---

## Recommended implementation approach

## Step 1 — Add Airflow image and service definitions

Introduce:

- `docker/airflow.Dockerfile`
- new services in `docker-compose.yml`
- shared Airflow env file
- local logs directory
- DAG and plugins mounts

Success means the containers start successfully.

---

## Step 2 — Initialize Airflow metadata

Use `airflow-init` to:

- migrate/init the Airflow DB
- create an admin user for local access

Success means the webserver and scheduler start against a valid DB.

---

## Step 3 — Add a minimal DAG

Add a simple `ping_runner_dag.py`.

Success means:

- DAG appears in UI
- manual trigger works
- task logs show successful connectivity to `runner`

---

## Step 4 — Add minimal helper client

Add `rs_foundry_client.py` inside `plugins/`.

Even if tiny, this creates the correct long-term pattern:

- DAGs stay thin
- API calling logic is shared
- future trigger/poll code has a stable home

---

## Step 5 — Document the contract boundary

Write the docs that clarify:

- Airflow is orchestration only
- Rust remains execution
- future DAGs must call the runner API
- transformation logic must not be added to DAG code

This matters because if the boundary is not documented now, Python DAG code will start absorbing business logic later.

---

## Acceptance criteria

Phase 7 is complete when all of the following are true:

### Platform

- Airflow services are defined in Docker Compose
- Airflow image builds successfully
- Airflow metadata DB initializes successfully
- Airflow UI is reachable locally
- scheduler and triggerer stay healthy

### Connectivity

- Airflow can resolve `runner` over the Compose network
- a validation DAG can successfully call the runner

### Repo structure

- Airflow directories exist in the repo
- DAGs, plugins, env, and logs are organized cleanly
- local logs persist in a mounted folder

### Documentation

- the orchestration boundary is documented
- the repo explains how to start Airflow locally
- the decision to call the runner API is documented explicitly

### Architecture discipline

- no transformation logic is added to DAG code
- no business logic is duplicated in Python
- Rust remains the only execution layer

---

## Deliverables

The expected deliverables for this phase are:

- updated `docker-compose.yml`
- `docker/airflow.Dockerfile`
- `orchestration/airflow/env/airflow.env`
- `orchestration/airflow/dags/ping_runner_dag.py`
- `orchestration/airflow/plugins/rs_foundry_client.py`
- `orchestration/airflow/README.md`
- `docs/airflow.md`
- `docs/phases/PHASE_7_AIRFLOW_FOUNDATION.md`

---

## Risks and controls

## Risk 1 — Airflow grows into a second execution platform

### Problem

DAG authors may start putting transformation logic into Python tasks.

### Control

Set the rule now:

- DAGs trigger and wait
- Rust executes all data logic

Document this in `docs/airflow.md`.

---

## Risk 2 — Airflow bypasses the Rust control plane

### Problem

It may seem convenient to invoke Rust bins directly from containers.

### Control

Use runner API calls as the standard path from the beginning.

Direct CLI/container execution remains a fallback only.

---

## Risk 3 — Local stack becomes too heavy too early

### Problem

Adding too many orchestration components at once creates friction.

### Control

Use:

- shared Postgres container
- LocalExecutor
- only the minimum Airflow services
- one validation DAG

---

## Risk 4 — Traceability becomes split

### Problem

Airflow and `rs-foundry` can create separate identities with no clear relationship.

### Control

In later phases, every Airflow-triggered execution must create a Rust `run_id` and store orchestration context in the run registry.

This phase lays the groundwork for that model.

---

## What this phase intentionally does not settle yet

This phase creates the foundation, but the following are deferred to later phases:

- full job trigger payload design
- `orchestration_context` request schema
- `run_id` propagation from Airflow into downstream tasks
- bronze-to-silver DAG composition
- conformed fan-in orchestration
- Airflow retries mapped to run registry semantics
- deferrable sensors for efficient long polling
- external production deployment shape beyond local Compose

These belong in the next Airflow integration phases.

---

## Recommended next phase

After this phase, the next step should be:

**Phase 8 — First real Airflow-triggered Rust job**

That phase should implement the first real thin DAG, preferably:

```
bronze_ref_ingest_dag.py
```

It should:

- call the Rust runner API
- receive a `run_id`
- poll run completion
- surface success/failure in Airflow
- preserve the `rs-foundry` control-plane model

---

## Summary

Phase 7 establishes the correct Airflow foundation for `rs-foundry` by:

- adding Airflow to the Docker-first local stack
- preserving Rust as the execution layer
- treating Airflow as orchestration only
- validating connectivity to the runner
- creating the repo structure for thin DAGs
- documenting the long-term control-plane boundary

This phase is successful when Airflow is fully wired into the local stack and ready to orchestrate real Rust jobs in the next phase.