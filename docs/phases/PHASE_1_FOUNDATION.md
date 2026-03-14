# PHASE_1_FOUNDATION.md

# Phase 1 — Foundation

## Objective

Establish the repo, module boundaries, config loading, local Docker stack, notebook sidecar, and baseline docs.

## Outcome

By the end of Phase 1, the team should have:

- a compiling Rust workspace
- a runnable local Compose stack
- environment-aware config
- a Postgres service available for future run tracking
- an optional notebook container for Evcxr/Jupyter
- clear docs showing where code belongs

---

## Recommended Phase 1 ticket order

## RS-010 — Initialize repo scaffold

This is the repo shape and baseline files.

### Goal

Create the base repository layout and baseline project files.

### Why it matters

This gives the team a stable skeleton so later work lands in predictable places and does not erode the architecture.

### Create

- `Cargo.toml`
- `Cargo.lock`
- `rust-toolchain.toml`
- `.env`
- `.env.example`
- `README.md`
- `DIRTREE.md`
- `docker-compose.yml`
- `Makefile`

### Create directories

- `docker/`
- `conf/base/`
- `conf/dev/`
- `conf/prod/`
- `data/raw/`
- `data/bronze/`
- `data/silver/`
- `data/gold/`
- `data/checkpoints/`
- `data/runs/`
- `docs/`
- `notebooks/bronze/`
- `notebooks/silver/`
- `notebooks/gold/`
- `src/`
- `src/bin/`
- `tests/`

### Acceptance criteria

- `cargo check` passes
- repo tree matches agreed structure
- empty crate compiles

### Implementation notes

- Start with a single crate/workspace root unless there is a strong reason to split early.
- Keep the initial scaffold lean and avoid premature complexity.

### How to run / test

```bash
cargo check
tree -L 3
