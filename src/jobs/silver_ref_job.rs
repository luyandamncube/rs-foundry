// src\jobs\silver_ref_job.rs
pub fn execute() {
    crate::pipelines::silver::standardize_ref::run_silver_ref_pipeline();
}
