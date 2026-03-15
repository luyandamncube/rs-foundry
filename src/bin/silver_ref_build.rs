// src\bin\silver_ref_build.rs
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()),
    )
    .init();

    let bronze_run_id = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("missing bronze_run_id argument"))?;

    let result = rs_foundry::jobs::silver_ref_job::execute(&bronze_run_id)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("silver_ref_build complete");
    println!("bronze_run_id: {}", result.bronze_run_id);
    println!("source_name  : {}", result.source_name);
    println!("bronze_path  : {}", result.bronze_path.display());
    println!("silver_path  : {}", result.silver_path.display());
    println!("record_count : {}", result.record_count);

    Ok(())
}