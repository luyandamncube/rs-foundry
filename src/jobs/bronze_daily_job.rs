// src\jobs\bronze_daily_job.rs
pub fn execute() {
    crate::pipelines::bronze::ingest_daily::run_bronze_daily_pipeline();
}
