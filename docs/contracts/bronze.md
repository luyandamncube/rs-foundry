# docs/contracts/bronze.md

# Bronze Layer Contracts

## Purpose

The bronze layer is the first operationalized layer after raw landing.

Its purpose is to:

- preserve close alignment to source structure
- attach ingestion and lineage metadata
- provide a stable first-stage dataset for downstream silver standardization
- perform basic quality validation before data moves further into the platform

The bronze layer is **not** intended to be fully cleaned, conformed, or consumer-ready. That responsibility belongs to silver and beyond.

---

## Bronze layer responsibilities

Bronze datasets are responsible for:

- parsing source inputs into typed Rust records
- preserving core source fields with minimal reshaping
- attaching metadata such as:
  - run ID
  - source name
  - source file
  - ingestion timestamp
  - load date
- writing reproducible outputs to the bronze storage path
- enforcing basic required-field and non-empty checks

Bronze datasets are **not** responsible for:

- deep deduplication
- business-level conformance
- cross-source joins
- heavy semantic cleanup
- consumer-specific reshaping

---

## Storage conventions

Bronze outputs are written under:

- `data/bronze/`

Current bronze dataset paths follow a deterministic pattern.

### Reference bronze dataset

```text
data/bronze/ref_example/load_date=<YYYY-MM-DD>/run_id=<uuid>/bronze_ref.json
```

### Daily Bronze Dataset
```bash
data/bronze/daily_example/load_date=<YYYY-MM-DD>/run_id=<uuid>/bronze_daily.json
```

### Path semantics

- `load_date` represents the processing/load partition date
- `run_id` is the execution identifier shared with the control-plane run registry
- the file name identifies the bronze dataset artifact for that source


## Raw-to-bronze traceability

Each bronze dataset should be traceable back to:

- the source dataset
- the raw landed file
- the run registry record
- the specific processing run that created it

This is why bronze records include operational metadata fields.

---

## Bronze dataset: `ref_example`

## Description

`ref_example` is the bronze representation of the reference-style source.

It preserves the source’s reference entity shape with minimal standardization and metadata enrichment.

## Source fields

The current bronze ref dataset originates from source records with fields such as:

- `ref_id`
- `ref_name`
- `ref_type`
- `is_active`
- `updated_at`

## Bronze ref schema

### Business/source-aligned fields

- `ref_id`
- `ref_name`
- `ref_type`
- `is_active`
- `updated_at`

### Bronze metadata fields

- `run_id`
- `source_name`
- `source_file`
- `ingested_at`
- `load_date`
- `record_hash`

## Field expectations

### `ref_id`
- required
- non-blank
- source identifier for the reference entity

### `ref_name`
- required
- non-blank
- source-provided display/name field

### `ref_type`
- required
- non-blank
- source-provided categorical/type field

### `is_active`
- required
- boolean activity flag from source

### `updated_at`
- optional
- source-provided update timestamp when available

### `run_id`
- required
- must match the control-plane run ID for the execution that created the bronze file

### `source_name`
- required
- identifies the logical source dataset

### `source_file`
- optional
- identifies the landed raw file where relevant

### `ingested_at`
- required
- timestamp at which bronze enrichment occurred

### `load_date`
- required
- date partition used for bronze storage

### `record_hash`
- optional in current MVP
- reserved for future row-level identity or replay logic

---

## Bronze dataset: `daily_example`

## Description

`daily_example` is the bronze representation of the daily/event-style source.

It preserves the event-like shape of the incoming data while attaching ingestion metadata.

## Source fields

The current bronze daily dataset originates from source records with fields such as:

- `event_id`
- `ref_id`
- `event_date`
- `metric_value`
- `status`
- `source_ts`

## Bronze daily schema

### Business/source-aligned fields

- `event_id`
- `ref_id`
- `event_date`
- `metric_value`
- `status`
- `source_ts`

### Bronze metadata fields

- `run_id`
- `source_name`
- `source_file`
- `ingested_at`
- `load_date`
- `record_hash`

## Field expectations

### `event_id`
- required
- non-blank
- event/business identifier for the daily record

### `ref_id`
- required
- non-blank
- reference identifier used later for conformance/joining

### `event_date`
- required
- date associated with the event/measurement

### `metric_value`
- required
- numeric value carried forward for later silver validation and downstream use

### `status`
- required
- non-blank
- source-provided event status

### `source_ts`
- optional
- source-origin timestamp when provided

### `run_id`
- required
- must match the control-plane run ID for the execution that created the bronze file

### `source_name`
- required
- identifies the logical source dataset

### `source_file`
- optional
- identifies the landed raw file where relevant

### `ingested_at`
- required
- timestamp at which bronze enrichment occurred

### `load_date`
- required
- date partition used for bronze storage

### `record_hash`
- optional in current MVP
- reserved for future row-level identity or replay logic

---

## Bronze quality rules

Bronze quality rules are intentionally basic.

The goal is to catch clearly invalid data without overreaching into silver responsibilities.

## Current bronze checks for `ref_example`

- dataset must not be empty
- `ref_id` must not be blank
- `ref_name` must not be blank
- `ref_type` must not be blank

## Current bronze checks for `daily_example`

- dataset must not be empty
- `event_id` must not be blank
- `ref_id` must not be blank
- `status` must not be blank

## Failure behavior

If bronze quality checks fail:

- the pipeline returns an error
- the bronze output is not treated as valid for downstream use
- API-driven execution records the run as failed in the run registry

---

## Bronze guarantees

The bronze layer currently guarantees:

- typed record parsing into Rust structs
- deterministic output paths
- run-linked lineage metadata
- basic required-field validation
- stable first-stage outputs suitable for silver processing

The bronze layer does **not** yet guarantee:

- uniqueness of business keys
- cross-source consistency
- semantic normalization
- deduplication
- downstream-ready conformance

Those guarantees are introduced in silver.

---

## Relationship to silver

Silver reads bronze outputs and is responsible for:

- trimming and normalization
- derived normalized fields
- stronger contract checks
- deduplication
- business-key conformance
- cross-source alignment

Bronze should therefore remain intentionally conservative:
- close to source
- operationally traceable
- minimally transformed

---

## Evolution notes

The current MVP bronze layer writes JSON artifacts for speed of implementation and debugging clarity.

Future evolution may include:

- parquet outputs instead of JSON
- stronger schema drift checks
- row-level record hashes
- manifest files
- raw/bronze reconciliation reporting
- source-specific schema contracts in config or docs

---

## Summary

The bronze layer is the platform’s first reliable operational data layer.

It exists to provide:

- typed source-to-dataset conversion
- deterministic lineage
- stable metadata enrichment
- basic validation
- a clean handoff into silver

Bronze is not the final consumer layer.

Its value comes from being:

- reproducible
- inspectable
- traceable
- stable enough for the next standardization stage
