# docs/handoff.md

# rs-foundry DS/ML Handoff Guide

## Purpose

This document explains how downstream data science, analytics, or feature-engineering work should consume outputs from `rs-foundry`.

It identifies:

- the recommended handoff dataset
- where that dataset lives
- what guarantees it currently has
- what it does **not** yet guarantee
- how to regenerate it
- how downstream users should begin working with it

This document is intended to make the MVP usable beyond the engineering implementation itself.

---

## Recommended downstream dataset

The primary downstream handoff dataset for the current MVP is:

- `silver_conformed.json`

This is the most useful handoff artifact because it combines:

- cleaned reference attributes from `silver_ref`
- cleaned daily/event records from `silver_daily`
- deterministic silver-layer contract enforcement
- conformed join logic on `ref_id`

It is the best current starting point for:

- exploratory data analysis
- notebook-based feature exploration
- analytics prototyping
- downstream DS/ML dataset design

---

## Dataset location

The conformed silver dataset is written under:

```text
data/silver/conformed/load_date=<YYYY-MM-DD>/ref_bronze_run_id=<uuid>/daily_bronze_run_id=<uuid>/silver_conformed.json
```

## Example
```text
data/silver/conformed/load_date=2026-03-15/ref_bronze_run_id=56e2508f-dd42-419d-a417-5b4d36ee51c7/daily_bronze_run_id=e358cb13-b4b9-4fe5-8685-02013cfc65ae/silver_conformed.json
```

## Path meaning

- `load_date` = date the conformed dataset was built

- `ref_bronze_run_id` = upstream bronze reference run used for the build

- `daily_bronze_run_id` = upstream bronze daily run used for the build

This path structure allows downstream consumers to trace the dataset back to the exact upstream processing runs.

## Dataset description

The conformed silver dataset is an event-level dataset enriched with reference attributes.

It combines:

- event-level measurements and statuses from the daily source
- cleaned and trusted reference metadata from the reference source

Join key:

- `ref_id`

Current join behavior:

- only daily records that find a matching silver reference record are included in the conformed dataset

This means the conformed dataset acts like an inner join from silver daily to silver ref.

---

## Current schema summary

The conformed silver dataset currently contains the following logical groups of fields.

## Reference fields

- `ref_id`
- `ref_name`
- `ref_type`
- `ref_name_normalized`
- `ref_type_normalized`
- `ref_is_active`

## Event / fact fields

- `event_id`
- `event_date`
- `metric_value`
- `status`
- `status_normalized`
- `is_positive_metric`
- `source_ts`

## Lineage / processing fields

- `ref_bronze_run_id`
- `daily_bronze_run_id`
- `silver_built_at`
- `load_date`

---

## Current guarantees

The current MVP silver handoff dataset guarantees the following.

### 1. It is built from silver, not bronze
The handoff dataset is not raw or minimally transformed operational data. It is built after:

- standardization
- normalization
- deduplication
- contract checking

### 2. Reference and daily inputs are cleaned
Before conformance, the upstream silver datasets apply:

- trimming of required string fields
- normalized lowercase helper fields
- deterministic deduplication
- required-field checks
- business-key uniqueness checks

### 3. Key business identifiers are enforced
The current silver contract logic enforces:

- `ref_id` uniqueness in silver reference
- `event_id` uniqueness in silver daily
- `event_id` uniqueness in silver conformed

### 4. Basic numeric sanity is enforced
The current silver layer checks that:

- `metric_value` is finite

### 5. Lineage is preserved
The conformed output includes enough lineage to trace back to:

- the upstream bronze reference run
- the upstream bronze daily run
- the silver build timestamp

### 6. Join output is deterministic
Given the same upstream inputs and current code paths, the conformed build should produce the same joined output and sorted ordering.

---

## Current non-guarantees

The MVP does **not** yet guarantee the following.

### 1. Historical dimension modeling
The current implementation does not provide:

- slowly changing dimension handling
- temporal dimension versioning
- effective-date logic

### 2. Advanced late-arriving data handling
The current implementation does not include:

- replay-aware merge logic
- late correction logic
- multi-version event reconciliation

### 3. Full missing-join diagnostics
The conformed build currently logs or implies missing joins operationally, but does not yet produce a rich structured unmatched-record artifact.

### 4. Columnar/parquet handoff format
The current MVP writes JSON artifacts for speed and debuggability. It does not yet provide:

- parquet handoff outputs
- dataset manifests
- catalog registration

### 5. Consumer-specific feature engineering
The conformed dataset is downstream-ready, but it is not yet a formal feature set. It does not yet include:

- rolling windows
- aggregated features
- lag features
- target labels
- train/validation/test splits

### 6. Full production orchestration
The current handoff can be generated deterministically, but it is still based on the MVP’s direct execution model rather than a full async orchestration platform.

### How to generate the handoff dataset

The current end-to-end build path is:

1. generate bronze reference output
2. generate bronze daily output
3. optionally generate silver ref and silver daily outputs
4. build the conformed silver dataset

Example CLI flow
```bash
cargo run --bin bronze_ref_ingest
cargo run --bin bronze_daily_ingest
cargo run --bin silver_ref_build <ref_bronze_run_id>
cargo run --bin silver_daily_build <daily_bronze_run_id>
cargo run --bin silver_conformed_build <ref_bronze_run_id> <daily_bronze_run_id>
```

### Practical note

The conformed build currently reads from bronze inputs and reconstructs silver-standardized records in-process before joining them. This is acceptable for the MVP, but later versions may read directly from materialized silver artifacts or formalized tables.

## How downstream users should read it

The simplest current usage pattern is:

- locate the latest conformed output path
- read `silver_conformed.json`
- load into a notebook, pandas DataFrame, DuckDB table, or future Rust-native analysis path

### Current expected consumer patterns

- notebook exploration
- Python/pandas analysis
- DuckDB local querying
- feature ideation
- dataset profiling

### Reading considerations

Because the current artifact is JSON:
- it is easy to inspect manually
- it is easy to load into notebooks
- it is not yet the most efficient format for scale

That trade-off is acceptable for the MVP.

---

## Recommended downstream starting point

Downstream DS/ML work should begin with the conformed silver dataset and treat it as:

- the default trusted input
- the default exploratory dataset
- the base table from which additional features can be derived

### Recommended first DS tasks

- profile nulls, cardinalities, and distributions
- inspect `status` and `status_normalized`
- inspect `metric_value`
- evaluate whether `ref_type` or `ref_is_active` are predictive/useful
- derive simple aggregate and temporal features
- define downstream target logic if relevant

---

## Suggested first feature ideas

Depending on downstream goals, likely first feature candidates include:

- one-hot or encoded `ref_type`
- one-hot or encoded `status_normalized`
- binary `ref_is_active`
- binary `is_positive_metric`
- normalized or bucketed `metric_value`
- grouped counts or summary measures by `ref_id`
- event frequency by date
- simple lag/rolling calculations in a future expanded dataset

These are not part of the current handoff contract; they are recommended next steps for downstream work.

---

## Recommended downstream contract usage

Downstream users should treat the silver conformed dataset as:

- trusted enough for exploration
- stable enough for MVP analytics work
- lineage-aware
- still evolving

### Practical guidance

Use the silver conformed dataset for:
- EDA
- prototyping
- initial modeling experiments
- feature brainstorming

Do not yet assume it is:
- final production feature-store output
- historically versioned truth
- fully governed enterprise data product

---

## Relationship to other datasets

### `bronze_ref.json`
Useful for:
- debugging source ingestion
- seeing minimally transformed reference records

Not recommended as the primary DS handoff.

### `bronze_daily.json`
Useful for:
- debugging event ingestion
- tracing raw event-level behavior

Not recommended as the primary DS handoff.

### `silver_ref.json`
Useful for:
- dimension profiling
- inspecting cleaned reference attributes

Helpful, but not the main downstream starting point.

### `silver_daily.json`
Useful for:
- inspecting cleaned event records
- testing event-level feature logic in isolation

Helpful, but still not the best single-entry handoff.

### `silver_conformed.json`
Recommended primary handoff because it is:
- joined
- cleaned
- deduplicated
- contract-checked
- more immediately useful

---

## Operational notes

When using the conformed silver dataset downstream, always keep track of:

- `ref_bronze_run_id`
- `daily_bronze_run_id`
- `load_date`

These fields make it possible to:
- reproduce a specific dataset state
- investigate upstream issues
- compare downstream experiments against exact upstream inputs

---

## Future handoff improvements

The handoff experience can be improved later with:

- parquet outputs
- a `latest/` pointer or manifest
- explicit unmatched-join report files
- richer schema docs
- sample notebook templates for DS consumers
- gold-layer feature tables
- formal train/validation/test dataset generation

These are all good next steps, but they are not required for the MVP handoff.

---

## Summary

For the current MVP, the recommended DS/ML handoff artifact is:

- `data/silver/conformed/.../silver_conformed.json`

It is the best available downstream dataset because it is:

- standardized
- deduplicated
- contract-checked
- lineage-aware
- enriched through reference/event conformance

This dataset should be treated as the primary starting point for:
- exploratory analysis
- early feature engineering
- downstream DS/ML prototyping

It is not yet the final future-state analytical platform output, but it is a strong and practical MVP handoff boundary.
