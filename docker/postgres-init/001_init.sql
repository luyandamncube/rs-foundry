-- docker\postgres-init\001_init.sql
CREATE TABLE IF NOT EXISTS runs (
    run_id UUID PRIMARY KEY,
    job_name TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    error_message TEXT
);
