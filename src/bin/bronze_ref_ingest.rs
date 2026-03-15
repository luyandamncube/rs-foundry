// src\bin\bronze_ref_ingest.rs
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()),
    )
    .init();

    let result = rs_foundry::jobs::bronze_ref_job::execute()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("bronze_ref_ingest complete");
    println!("run_id      : {}", result.run_id);
    println!("source_name : {}", result.source_name);
    println!("raw_path    : {}", result.raw_path.display());
    println!("bronze_path : {}", result.bronze_path.display());
    println!("record_count: {}", result.record_count);

    Ok(())
}
