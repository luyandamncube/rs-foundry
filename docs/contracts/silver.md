# Silver Layer Contracts

## Purpose

The silver layer is the platform’s trusted, standardized data layer.

Its purpose is to:

- clean and normalize bronze outputs
- enforce stronger data contracts than bronze
- deduplicate records using deterministic business-key rules
- align related datasets into a reusable downstream-facing structure
- provide stable datasets for analytics, DS/ML exploration, and future gold builds

The silver layer is where data becomes meaningfully more trustworthy.

---

## Silver layer responsibilities

Silver datasets are responsible for:

- trimming and normalizing source-derived string fields
- standardizing categorical values where appropriate
- preserving lineage back to bronze runs
- enforcing stronger required-field rules
- enforcing business-key uniqueness
- deduplicating records deterministically
- building conformed cross-source datasets

Silver datasets are **not** primarily responsible for:

- preserving full source fidelity
- raw replay or landing concerns
- API/job orchestration
- consumer-specific aggregates
- final presentation-layer shaping

Those concerns belong to raw/bronze, control-plane, or gold layers respectively.

---

## Storage conventions

Silver outputs are written under:

- `data/silver/`

Current silver dataset paths follow deterministic patterns.

### Silver reference dataset

```text
data/silver/ref_example/load_date=<YYYY-MM-DD>/bronze_run_id=<uuid>/silver_ref.json
```

### Silver daily dataset

```text
data/silver/daily_example/load_date=<YYYY-MM-DD>/bronze_run_id=<uuid>/silver_daily.json
```

### Silver conformed dataset

```text
data/silver/conformed/load_date=<YYYY-MM-DD>/ref_bronze_run_id=<uuid>/daily_bronze_run_id=<uuid>/silver_conformed.json
```

## Bronze-to-silver traceability

Each silver dataset should be traceable back to:

- the upstream bronze dataset
- the bronze run ID that produced it
- the source name and source file where relevant
- the time the bronze data was ingested
- the time the silver dataset was built

This is why silver records preserve lineage fields rather than only keeping cleaned business fields.

---

## Silver dataset: `ref_example`

## Description

`silver_ref.json` is the standardized and deduplicated silver representation of the reference-style source.

It is built from `bronze_ref.json` and is intended to provide a trustworthy reference dimension for downstream joins and conformed outputs.

## Silver ref schema

### Business fields

- `ref_id`
- `ref_name`
- `ref_type`
- `ref_name_normalized`
- `ref_type_normalized`
- `is_active`
- `updated_at`

### Lineage and processing fields

- `bronze_run_id`
- `source_name`
- `source_file`
- `bronze_ingested_at`
- `silver_built_at`
- `load_date`

## Standardization behavior

### `ref_id`
- trimmed
- treated as the business key
- expected to be unique in silver

### `ref_name`
- trimmed
- preserved in business-facing form

### `ref_type`
- trimmed
- preserved in business-facing form

### `ref_name_normalized`
- lowercase normalized form of `ref_name`
- used for consistent downstream matching/filtering

### `ref_type_normalized`
- lowercase normalized form of `ref_type`
- used for consistent downstream matching/filtering

### `is_active`
- carried forward from bronze
- preserved as a boolean signal for downstream use

### `updated_at`
- carried forward from bronze when available
- used in deduplication preference logic

## Deduplication rule

Silver ref records are deduplicated on:

- `ref_id`

When duplicate `ref_id` values are present, the preferred record is chosen using:

1. latest `updated_at`
2. if `updated_at` is unavailable or tied, latest `bronze_ingested_at`

This ensures a deterministic “latest known reference state” outcome.

## Silver ref contract rules

Current enforced rules include:

- dataset must not be empty
- `ref_id` must be non-blank
- `ref_name` must be non-blank
- `ref_type` must be non-blank
- `ref_name_normalized` must be non-blank
- `ref_type_normalized` must be non-blank
- `ref_id` must be unique

---

## Silver dataset: `daily_example`

## Description

`silver_daily.json` is the standardized and deduplicated silver representation of the daily/event-style source.

It is built from `bronze_daily.json` and is intended to provide a trustworthy event/fact-like dataset for downstream joins and analysis.

## Silver daily schema

### Business fields

- `event_id`
- `ref_id`
- `event_date`
- `metric_value`
- `status`
- `status_normalized`
- `is_positive_metric`
- `source_ts`

### Lineage and processing fields

- `bronze_run_id`
- `source_name`
- `source_file`
- `bronze_ingested_at`
- `silver_built_at`
- `load_date`

## Standardization behavior

### `event_id`
- trimmed
- treated as the business key
- expected to be unique in silver

### `ref_id`
- trimmed
- preserved for downstream conformance with reference data

### `event_date`
- preserved as the event/business date

### `metric_value`
- preserved as numeric fact data
- must remain finite

### `status`
- trimmed
- preserved in business-facing form

### `status_normalized`
- lowercase normalized form of `status`
- supports more consistent downstream grouping/filtering

### `is_positive_metric`
- derived boolean flag
- indicates whether `metric_value > 0.0`

### `source_ts`
- carried forward when available
- used in deduplication preference logic

## Deduplication rule

Silver daily records are deduplicated on:

- `event_id`

When duplicate `event_id` values are present, the preferred record is chosen using:

1. latest `source_ts`
2. if `source_ts` is unavailable or tied, latest `bronze_ingested_at`

This ensures a deterministic “latest event version” outcome.

## Silver daily contract rules

Current enforced rules include:

- dataset must not be empty
- `event_id` must be non-blank
- `ref_id` must be non-blank
- `status` must be non-blank
- `status_normalized` must be non-blank
- `metric_value` must be finite
- `event_id` must be unique

---

## Silver dataset: `conformed`

## Description

`silver_conformed.json` is the downstream-facing conformed silver dataset created by aligning:

- `silver_ref`
- `silver_daily`

The conformed build joins daily records to reference records on:

- `ref_id`

This creates an enriched event-level dataset that combines fact-like daily measures with trustworthy reference attributes.

## Silver conformed schema

### Joined reference fields

- `ref_id`
- `ref_name`
- `ref_type`
- `ref_name_normalized`
- `ref_type_normalized`
- `ref_is_active`

### Daily/event fields

- `event_id`
- `event_date`
- `metric_value`
- `status`
- `status_normalized`
- `is_positive_metric`
- `source_ts`

### Lineage and processing fields

- `ref_bronze_run_id`
- `daily_bronze_run_id`
- `silver_built_at`
- `load_date`

## Join rule

The conformed dataset currently performs an inner-style join behavior:

- only daily records with matching `ref_id` values in silver reference are included

If a daily record does not find a matching reference row, it is not emitted into the conformed dataset in the current MVP behavior.

## Conformed contract rules

Current enforced rules include:

- dataset must not be empty
- `event_id` must be non-blank
- `ref_id` must be non-blank
- `ref_name` must be non-blank
- `ref_type` must be non-blank
- `ref_name_normalized` must be non-blank
- `ref_type_normalized` must be non-blank
- `status` must be non-blank
- `status_normalized` must be non-blank
- `metric_value` must be finite
- `event_id` must be unique

---

## Silver guarantees

The silver layer currently guarantees:

- trimmed and normalized string fields
- deterministic deduplication for key datasets
- stronger required-field enforcement than bronze
- stable lineage back to bronze runs
- downstream-friendly conformed outputs
- explicit business-key uniqueness checks for current silver datasets

The silver layer does **not** yet guarantee:

- full historical slowly changing dimension handling
- late-arriving correction logic
- multi-source survivorship rules beyond current deterministic logic
- advanced anomaly detection
- consumer-specific aggregations

Those can evolve later if the platform expands.

---

## Relationship to bronze

Bronze remains:

- close to source
- minimally transformed
- operationally traceable

Silver builds on bronze by adding:

- normalization
- business-key rules
- deduplication
- stronger contracts
- conformance between datasets

A dataset should move from bronze to silver when the platform needs it to be trusted and reusable, not merely landed.

---

## Relationship to gold

Gold is expected to build on silver by adding:

- consumer-specific shaping
- aggregates
- feature-oriented logic
- reporting/analytics-ready semantics

Silver should therefore remain:

- trustworthy
- reusable
- relatively general-purpose

rather than overly tailored to one downstream consumer.

---

## Failure behavior

If silver contract checks fail:

- the silver pipeline returns an error
- the silver output should not be treated as valid for downstream use
- CLI/API surfaces should surface the error clearly
- upstream bronze data remains available for debugging and replay

This behavior is intentional: silver is a trust boundary.

---

## Evolution notes

The current MVP silver layer writes JSON artifacts for speed of implementation and debugging clarity.

Future evolution may include:

- parquet outputs instead of JSON
- configurable contract definitions
- more explicit join diagnostics for conformed builds
- unmatched-reference reporting
- richer business-key rules
- dataset manifests
- contract versioning

---

## Summary

The silver layer is where `rs-foundry` turns operational data into trusted reusable datasets.

It exists to provide:

- standardized fields
- deterministic deduplication
- stronger validation
- conformed multi-source outputs
- a clean handoff into analytics and DS/ML exploration

If bronze makes data operational, silver makes it trustworthy.
