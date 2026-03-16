-- docker/postgres-init/002_runs_orchestration_metadata.sql

ALTER TABLE runs
ADD COLUMN IF NOT EXISTS orchestrator TEXT,
ADD COLUMN IF NOT EXISTS orchestrator_dag_id TEXT,
ADD COLUMN IF NOT EXISTS orchestrator_dag_run_id TEXT,
ADD COLUMN IF NOT EXISTS orchestrator_task_id TEXT,
ADD COLUMN IF NOT EXISTS orchestrator_try_number INTEGER,
ADD COLUMN IF NOT EXISTS upstream_run_ids JSONB NOT NULL DEFAULT '[]'::jsonb;