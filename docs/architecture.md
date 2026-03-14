# docs/architecture.md

# rs-foundry Architecture

## Overview

`rs-foundry` is a Rust-first data platform MVP designed around a simple rule:

- **Rust owns the runtime**
- **notebooks support design-time exploration only**

The platform is intended to provide a clean, production-minded foundation for layered data processing with clear separation between:

- **control plane**
- **data plane**
- **design-time notebook sidecar**

It is built to support a layered data architecture:

- `raw`
- `bronze`
- `silver`
- `gold`

The initial MVP focuses on:

- strong repo structure
- containerized local development
- reproducible execution
- clean module boundaries
- testability
- future extensibility for orchestration and downstream DS/ML use

---

## Design principles

## 1. Rust-first runtime

All important runtime logic belongs in Rust. This includes:

- ingestion
- schema mapping
- transformation
- metadata enrichment
- file writes
- quality checks
- orchestration logic
- run tracking
- API handling
- worker execution

This ensures the system remains:

- performant
- testable
- maintainable
- explicit in behavior

## 2. Notebook sidecar, not notebook platform

Jupyter + Evcxr may be used for:

- source inspection
- schema exploration
- transform prototyping
- output validation
- debugging

However, notebooks must not become:

- the only execution path
- the only place where transformation logic lives
- part of the production runtime

Any logic worth keeping must be moved into `src/`.

## 3. Production-minded structure

The repository is organized so that:

- reusable logic lives in library modules
- binaries are thin operational entry points
- layered storage is explicit
- configuration is environment-aware
- runtime behavior is reproducible in Docker

## 4. MVP first, extensible later

The MVP favors:

- simple direct execution
- clear boundaries
- practical storage choices
- straightforward orchestration

It intentionally avoids overengineering in early phases while leaving room for:

- async workers
- queue-backed dispatch
- stronger observability
- richer metadata handling
- DS/ML downstream workflows

---

## High-level architecture

The system consists of three conceptual areas:

1. **Control plane**
2. **Data plane**
3. **Notebook sidecar**

---

## Control plane

The control plane is responsible for managing execution rather than transforming data directly.

Its responsibilities include:

- receiving job requests
- validating request payloads
- creating run records
- dispatching jobs
- exposing run status
- managing job lifecycle state

### Primary control-plane modules

- `src/api/`
- `src/orchestration/`

### Main control-plane runtime entry point

- `src/bin/runner_api.rs`

### Future optional control-plane extensions

- scheduler
- queue-backed dispatch
- richer run filtering
- retry policies
- multi-job orchestration

---

## Data plane

The data plane performs the actual data work.

Its responsibilities include:

- reading source inputs
- landing raw payloads
- mapping schemas
- standardizing records
- enriching metadata
- running quality checks
- writing layered outputs

### Primary data-plane modules

- `src/io/`
- `src/pipelines/`
- `src/jobs/`
- `src/quality/`

### Main data-plane runtime entry points

- `src/bin/bronze_ref_ingest.rs`
- `src/bin/bronze_daily_ingest.rs`
- `src/bin/silver_ref_build.rs`
- `src/bin/silver_conformed_build.rs`

### Data-plane design intent

The data plane should be:

- deterministic where practical
- explicit about schemas
- clear about layer responsibilities
- reusable across CLI, API, and worker-driven execution paths

---

## Notebook sidecar

The notebook sidecar exists for design-time work only.

### Intended uses

- inspect source files
- understand source schema shape
- prototype mappings
- validate bronze and silver outputs
- explore data issues before codifying logic in Rust

### Non-goals

The notebook sidecar must not:

- own production transformations
- become a hidden dependency of job execution
- contain business logic that does not exist in Rust modules

### Notebook location

- `notebooks/`

### Example workflow

1. inspect sample source in notebook
2. prototype a transform
3. validate expected output
4. move stable logic into `src/pipelines/` or `src/jobs/`
5. keep notebook only as a validation/design artifact

---

## Layered data architecture

The platform uses a layered storage model to make data maturity explicit.

## Raw

### Purpose
Preserve source fidelity for replay, debugging, and auditability.

### Characteristics
- source-shaped
- minimally altered
- useful for traceability
- stored before meaningful transformation

### Typical operations
- source landing
- file capture
- payload archival
- source metadata capture

### Location
- `data/raw/`

---

## Bronze

### Purpose
Create minimally standardized datasets with operational metadata.

### Characteristics
- close to source shape
- basic schema handling
- ingestion metadata added
- initial validations applied

### Typical operations
- parse source records
- attach run metadata
- add ingestion timestamps
- land parquet outputs
- run basic quality checks

### Location
- `data/bronze/`

---

## Silver

### Purpose
Produce cleaned, standardized, conformed datasets ready for broader reuse.

### Characteristics
- stronger typing
- cleaned values
- deduplication
- business-key logic
- stronger contract checks
- conformed joins or aligned datasets

### Typical operations
- normalize column names
- cast data types
- enforce null and uniqueness rules
- deduplicate
- conform across inputs

### Location
- `data/silver/`

---

## Gold

### Purpose
Provide downstream-facing datasets for analytics, feature engineering, or consumer-specific use.

### Characteristics
- consumer-oriented shape
- stable semantics
- ready for BI, analytics, or DS/ML workflows

### Typical operations
- aggregations
- feature derivations
- consumer-specific shaping

### Location
- `data/gold/`

---

## Runtime execution model

The runtime is designed to support both direct and async execution paths.

## Direct execution path (MVP default)

This is the primary execution mode for the MVP.

### Flow

1. client calls runner API
2. API validates request
3. API creates run record
4. API invokes Rust job directly
5. job executes pipeline logic
6. data is written to the correct layer
7. quality checks run inline
8. run record is updated
9. API returns result metadata

### Why this is the default

- simpler to build
- easier to debug
- fewer moving parts
- enough for MVP control-plane credibility

---

## Async execution path (future phase)

This path is planned for later phases.

### Flow

1. client calls runner API
2. API validates request
3. API creates run record
4. API enqueues work item
5. worker consumes work item
6. worker executes Rust pipeline
7. run status updates through lifecycle
8. client polls run status endpoint

### Why this comes later

- adds lifecycle complexity
- adds queue/worker coordination concerns
- is not required to prove the architecture in the MVP

---

## Run tracking model

Run tracking is part of the control-plane backbone.

### Responsibilities

- assign run IDs
- record job type
- track status transitions
- record timestamps
- store error messages
- support run inspection

### Initial MVP status model

- `created`
- `running`
- `succeeded`
- `failed`

### Future async status additions

- `queued`
- `cancelled`
- `retrying`

---

## Storage model

The MVP uses pragmatic local storage choices.

## Layered dataset storage

Primary dataset storage is filesystem-backed and organized under:

- `data/raw/`
- `data/bronze/`
- `data/silver/`
- `data/gold/`

This keeps the local MVP simple while preserving a data-lake style layout.

## Run metadata persistence

Run metadata is intended to be stored in Postgres for:

- run lookup
- status inspection
- future orchestration extensions

## Future storage evolution

Later versions may add:

- object-store-native paths
- MinIO or S3-compatible storage
- richer metadata indexing
- dataset catalogs

---

## Configuration model

Configuration is environment-aware and centralized.

### Goals

- avoid scattered hardcoded values
- support local dev and future deployment environments
- make paths and service settings explicit

### Current config structure

- `conf/base/`
- `conf/dev/`
- `conf/prod/`

### Initial config categories

- `app.toml`
- `sources.toml`
- `sinks.toml`
- `quality.toml`

### Configuration responsibilities

- service host/port
- environment name
- data root
- config root
- Postgres connection settings
- future source and sink settings
- future quality settings

---

## Repo structure and architectural ownership

The repository is organized so responsibilities are visible from the tree.

## Shared foundation modules

### `src/core/`
Common shared types, metadata types, schemas, and errors.

### `src/config/`
Settings loading and centralized layered path handling.

---

## Control-plane modules

### `src/api/`
HTTP routes, handlers, request/response models, and app state.

### `src/orchestration/`
Run registry, queue abstraction, scheduler placeholders, and dispatch logic.

---

## Data-plane modules

### `src/io/`
Input/output helpers such as files, HTTP, parquet, object store, and Postgres access.

### `src/pipelines/`
Layer-specific transformation logic for bronze, silver, and gold.

### `src/jobs/`
Operational job wrappers that connect execution intent to pipeline logic.

### `src/quality/`
Quality rules, contract checks, and quality reports.

---

## Operational binaries

Operational entry points live under `src/bin/`.

This keeps the runtime surfaces explicit while preventing business logic from leaking into `main.rs` files.

Examples:

- `runner_api.rs`
- `worker.rs`
- `bronze_ref_ingest.rs`
- `bronze_daily_ingest.rs`
- `silver_ref_build.rs`
- `silver_conformed_build.rs`

---

## Testing philosophy

Testing is part of the architecture, not an afterthought.

## Unit tests

Used for:

- config parsing
- path helpers
- schema serialization/deserialization
- normalization logic
- quality rule logic

## Integration tests

Used for:

- end-to-end bronze writes
- end-to-end silver builds
- run-registry persistence
- future queue/worker execution

## API tests

Used for:

- health endpoint
- job submission
- validation failures
- run-status endpoints

## Data-quality tests

Used for:

- required fields
- uniqueness
- schema drift handling
- duplicate handling
- contract enforcement

---

## Observability direction

Observability is intentionally lightweight in the early MVP.

## Initial approach

- structured logs
- run IDs in logs
- stage-level logging around pipeline execution
- readable error messages

## Future enhancements

- metrics
- stage durations
- richer run inspection
- dashboarding
- pipeline lineage visibility

---

## MVP scope boundaries

## In scope

- Rust control plane
- Rust data plane
- layered data layout
- local Docker Compose development
- Postgres-backed run tracking foundation
- notebook sidecar for design-time work
- direct execution path
- bronze and silver centered MVP

## Out of scope

- full DAG orchestration
- distributed scheduling
- advanced retries
- Kubernetes deployment
- production authn/authz
- streaming-first complexity
- heavy metadata platform features

---

## Recommended implementation sequence

The architecture is intended to be built in this order:

1. repo scaffold
2. module boundaries
3. config and path handling
4. Docker Compose stack
5. notebook sidecar
6. docs
7. bronze ingestion
8. control plane direct execution
9. silver standardization
10. async worker path
11. observability and DS handoff

This keeps the project grounded in a stable foundation before introducing orchestration complexity.

---

## Summary

`rs-foundry` is designed to be a clean, Rust-first layered data platform where:

- the **control plane** manages execution
- the **data plane** performs ingestion and transformation
- the **notebook sidecar** supports exploration but never owns runtime logic

The architecture intentionally prioritizes:

- clarity
- maintainability
- testability
- containerized local development
- future extensibility

The MVP is not trying to be a full platform on day one.

It is trying to establish the correct shape so that future ingestion, orchestration, and DS/ML handoff work can grow on top of a stable foundation.
