// src\jobs\silver_conformed_job.rs
pub fn execute() {
    crate::pipelines::silver::build_conformed::run_silver_conformed_pipeline();
}
