// src\jobs\bronze_ref_job.rs
pub fn execute() {
    crate::pipelines::bronze::ingest_ref::run_bronze_ref_pipeline();
}
