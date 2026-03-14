# README.md

# rs-foundry

`rs-foundry` is a Rust-first data platform MVP.

It is designed around a simple rule:

- **Rust owns the runtime**
- **notebooks support design-time exploration only**

The platform is intended to provide a clean, production-minded foundation for:

- ingestion
- schema mapping
- transformation
- metadata enrichment
- quality checks
- layered data writes
- run tracking
- API-driven execution
- future worker-based orchestration

---

## Core principles

- Rust owns the data plane
- Rust owns the control plane
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
- dispatch decisions
- run status exposure
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
- `src/bin/silver_conformed_build.rs`

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
- cleaned, typed, deduplicated, conformed datasets
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

## Current Phase 1 foundation scope

Phase 1 establishes:

- repo scaffold
- Rust module boundaries
- environment-aware config
- local Docker Compose stack
- Postgres service
- optional notebook sidecar
- architecture docs

Phase 1 does **not** yet focus on full ingestion behavior or worker orchestration.

---

## Repo layout

```text
rs-foundry/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── docker-compose.yml
├── .env
├── .env.example
├── README.md
├── DIRTREE.md
├── docker/
├── conf/
├── data/
├── docs/
├── notebooks/
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

```bash
rustc --version
cargo --version
cargo check
cargo test
```
## 2. Build + Test locally
```bash
cargo check
cargo test
cargo test --test bronze_tests
cargo test --test silver_tests
cargo test --test quality_tests
cargo test --test api_tests

```

## 3. Run API Runner Locally
```bash
cargo run --bin runner_api

# check health
curl http://localhost:8080/health

# expected response
{"status":"ok","service":"rs-foundry"}

```

Docker Compose usage
## 4. Start the local stack
```bash
docker compose up --build
```

This starts:
- postgres
- runner

Optional profile-based services:
- worker
- notebook

Start notebook profile
```bash
docker compose --profile notebook up --build notebook
```
Start worker profile
```bash
docker compose --profile worker up --build worker
```


## Configuration

`rs-foundry` uses a layered, environment-aware configuration model so runtime behavior is explicit and reproducible.

Configuration is loaded in this order:

1. `conf/base/app.toml`
2. `conf/<APP_ENV>/app.toml`
3. environment variables from `.env` or the shell

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
- `sources.toml` — source definitions for future ingestion work
- `sinks.toml` — sink/output definitions for future write targets
- `quality.toml` — quality-rule defaults for future pipeline checks

### Current runtime settings

The current Phase 1 setup focuses mainly on application and local infrastructure settings, including:

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

### Example flow

If `APP_ENV=dev`, configuration is resolved roughly as:

- load defaults from `conf/base/app.toml`
- apply overrides from `conf/dev/app.toml`
- apply any matching environment-variable overrides

### Why this approach

This model keeps configuration:

- centralized
- easy to reason about
- environment-specific without duplicating everything
- safer than scattering hardcoded values across the codebase

### Rule of thumb

- put stable shared defaults in `conf/base/`
- put environment-specific overrides in `conf/dev/` or `conf/prod/`
- use environment variables for deployment/runtime overrides
- do not hardcode runtime paths, ports, or connection settings in Rust modules

## Data directories

`rs-foundry` uses an explicit layered local data layout under `data/` so each stage of data maturity is visible and predictable.

### Root data path

The root data path is controlled through configuration, typically via:

- `DATA_ROOT`
- path helpers in `src/config/paths.rs`

This ensures storage locations are centralized rather than hardcoded throughout the codebase.

### Directory layout

- `data/raw/`
- `data/bronze/`
- `data/silver/`
- `data/gold/`
- `data/checkpoints/`
- `data/runs/`

### `data/raw/`

Purpose:

- preserve source fidelity
- store landed source payloads before meaningful transformation
- support replay, debugging, and traceability

Typical contents:

- raw files from sources
- source-shaped payload captures
- minimally altered landed inputs

### `data/bronze/`

Purpose:

- store minimally standardized datasets
- attach ingestion metadata
- preserve close alignment to source structure

Typical contents:

- parsed source outputs
- parquet datasets with metadata such as run ID, source name, and ingest timestamp
- first-stage validated data

### `data/silver/`

Purpose:

- store cleaned, typed, deduplicated, and conformed datasets
- provide stronger guarantees for downstream use

Typical contents:

- standardized datasets
- cleaned and normalized outputs
- conformed datasets built from multiple bronze inputs

### `data/gold/`

Purpose:

- store consumer-facing datasets for analytics, reporting, or feature-oriented downstream use

Typical contents:

- curated outputs
- aggregates
- feature-ready or analysis-ready datasets

### `data/checkpoints/`

Purpose:

- support future pipeline checkpointing and resumability
- hold intermediate execution state where needed

Typical contents:

- checkpoint markers
- pipeline progress artifacts
- future state-tracking files for incremental jobs

### `data/runs/`

Purpose:

- support local run-oriented artifacts outside the database-backed run registry
- keep run-specific output or diagnostic files grouped by execution

Typical contents:

- local run logs
- temporary execution artifacts
- future run manifests or exported diagnostics

### Why this layout exists

This structure makes the platform easier to reason about because it separates:

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

Outcome:

- a compiling Rust workspace
- a runnable local development stack
- clear separation between control plane, data plane, and notebook sidecar

### Phase 2 — Bronze ingestion MVP

Focus:

- raw landing
- bronze reference ingestion
- bronze daily or event-style ingestion
- metadata enrichment
- initial schema validation
- basic quality checks

Outcome:

- one or two realistic ingestion paths
- raw and bronze outputs written to the layered local storage layout
- CLI-driven execution paths for ingestion jobs

### Phase 3 — Rust control plane MVP

Focus:

- runner API
- job submission endpoints
- run registry
- direct job execution from the API
- run status visibility
- API tests

Outcome:

- a usable control-plane entry point
- direct execution of bronze jobs through the runner API
- persistent run tracking foundation

### Phase 4 — Silver standardization MVP

Focus:

- silver standardization logic
- normalization and type cleanup
- deduplication
- contract checks
- conformed silver dataset build

Outcome:

- stronger, downstream-ready silver datasets
- a clear bridge between ingestion and analytics or DS/ML use

### Phase 5 — Async orchestration and worker path

Focus:

- queue abstraction
- worker service
- async run lifecycle
- queued dispatch
- worker/orchestration tests

Outcome:

- separation of job submission and execution
- optional async execution path for longer-running jobs
- improved extensibility for future scaling

### Phase 6 — Observability, polish, and DS handoff

Focus:

- structured logging
- improved run diagnostics
- data contract documentation
- curated notebooks for profiling and validation
- downstream-facing handoff outputs

Outcome:

- a more operationally usable MVP
- better visibility into runs and failures
- clearer handoff into future DS/ML workflows

### Why this sequence

The phases are ordered to avoid premature complexity:

- foundation first
- then ingestion
- then control-plane execution
- then stronger data quality and conformance
- then async orchestration
- then polish and handoff

This keeps the MVP grounded in a stable architecture before introducing more complex runtime behavior.
