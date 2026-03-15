# docs/workflow.md

# Workflow

## Step 1 — Create the notebook first

Start with a profiling notebook under:

```text
notebooks/bronze/00_source_profile_<source>.ipynb
```

Recommended notebook outline:

1. add light deps only
    - `serde`
    - `serde_json`
2. fetch data
    - prefer shell `curl` via `std::process::Command`
    - avoid heavy HTTP crates in notebooks where possible
3. parse JSON into a source-shaped struct
4. profile the source
    - row count
    - field presence
    - blank/null checks
    - uniqueness of candidate keys
    - simple distributions
5. write notebook conclusions
    - business key
    - required fields
    - likely bronze checks
    - likely silver normalization rules

Notebook purpose:

- inspect
- profile
- validate assumptions

Notebook non-goal:

- do not let important business logic live only in the notebook

---

## Step 2 — Create the backend objects

Once the notebook findings are stable, move the real logic into Rust.

Recommended object pattern:

### Core source/domain type

```
src/core/types.rs
```

Naming:

- `<SourceName>SourceRecord`

Example:

- `CommentSourceRecord`

Use for:

- source-shaped payload parsing

### Bronze record type

```
src/core/metadata.rs
```

Naming:

- `Bronze<SourceName>Record`

Example:

- `BronzeCommentRecord`

Use for:

- source fields plus bronze lineage/metadata

### Silver record type

```
src/core/metadata.rs
```

Naming:

- `Silver<SourceName>Record`

Example:

- `SilverCommentRecord`

Use for:

- cleaned, normalized, deduplicated fields

### Schema helpers

```
src/core/schemas.rs
```

Naming:

- `bronze_<source>_schema()`
- `silver_<source>_schema()`

Example:

- `bronze_comments_schema()`
- `silver_comments_schema()`

### Bronze quality checks

```
src/quality/checks.rs
```

Naming:

- `validate_bronze_<source>_records()`

Example:

- `validate_bronze_comment_records()`

### Silver contracts

```
src/quality/contracts.rs
```

Naming:

- `validate_silver_<source>_records()`

Example:

- `validate_silver_comment_records()`

### Bronze pipeline

```
src/pipelines/bronze/ingest_<source>.rs
```

Example:

- `src/pipelines/bronze/ingest_comments.rs`

### Silver pipeline

```
src/pipelines/silver/standardize_<source>.rs
```

Example:

- `src/pipelines/silver/standardize_comments.rs`

### Job wrappers

```
src/jobs/
```

Naming:

- `bronze_<source>_job.rs`
- `silver_<source>_job.rs`

Example:

- `bronze_comments_job.rs`
- `silver_comments_job.rs`

### CLI bins

```
src/bin/
```

Naming:

- `bronze_<source>_ingest.rs`
- `silver_<source>_build.rs`

Example:

- `bronze_comments_ingest.rs`
- `silver_comments_build.rs`

---

## Step 3 — Run the Rust jobs and validate outputs

Once the backend objects and pipelines exist, run the real Rust jobs rather than relying on notebook-only logic.

### Bronze run

Example:

```bash
cargo run --bin bronze_<source>_ingest
```

Concrete example:

```bash
cargo run --bin bronze_comments_ingest
```

Expected output shape:

```
<job> complete
run_id      : <uuid>
source_name : <source>
raw_path    : ./data/raw/<source>/load_date=<YYYY-MM-DD>/run_id=<uuid>/<raw_file>
bronze_path : ./data/bronze/<source>/load_date=<YYYY-MM-DD>/run_id=<uuid>/<bronze_file>
record_count: <n>
```

Expected bronze artifacts:

```
data/raw/<source>/load_date=<YYYY-MM-DD>/run_id=<uuid>/<raw_file>
data/bronze/<source>/load_date=<YYYY-MM-DD>/run_id=<uuid>/<bronze_file>
```

### Silver run

Use the bronze run ID returned from the bronze job.

Example:

```bash
cargo run --bin silver_<source>_build <bronze_run_id>
```

Concrete example:

```bash
cargo run --bin silver_comments_build <bronze_run_id>
```

Expected output shape:

```
<job> complete
bronze_run_id: <uuid>
source_name  : <source>
bronze_path  : ./data/bronze/<source>/load_date=<YYYY-MM-DD>/run_id=<uuid>/<bronze_file>
silver_path  : ./data/silver/<source>/load_date=<YYYY-MM-DD>/bronze_run_id=<uuid>/<silver_file>
record_count : <n>
```

Expected silver artifacts:

```
data/silver/<source>/load_date=<YYYY-MM-DD>/bronze_run_id=<uuid>/<silver_file>
```

### Conformed run (if the source participates in conformance)

If the source is part of a conformed build, run the relevant conformed job after the upstream bronze/silver inputs exist.

Example pattern:

```bash
cargo run --bin silver_conformed_build <ref_bronze_run_id> <daily_bronze_run_id>
```

Expected conformed artifacts:

```
data/silver/conformed/load_date=<YYYY-MM-DD>/ref_bronze_run_id=<uuid>/daily_bronze_run_id=<uuid>/silver_conformed.json
```

### Validation after the Rust run

After the jobs complete:

1. reopen the produced files in the notebook
2. profile the actual bronze or silver outputs
3. confirm:
    - row counts
    - required fields
    - key uniqueness
    - normalization behavior
    - lineage fields
4. update Rust logic if needed
5. rerun the real job

This keeps the notebook as a validation surface while Rust remains the production execution path.

---

## Rule of thumb

- notebook discovers the rules
- Rust backend owns the rules
- notebook validates the outputs after the Rust pipeline runs

## Short loop

1. profile source in notebook
2. decide rules
3. implement rules in Rust
4. run real pipeline
5. reopen output in notebook
6. validate result