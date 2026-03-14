// src\bin\bronze_daily_ingest.rs
fn main() -> anyhow::Result<()> {
    let result = rs_foundry::jobs::bronze_daily_job::execute()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("bronze_daily_ingest complete");
    println!("run_id      : {}", result.run_id);
    println!("source_name : {}", result.source_name);
    println!("raw_path    : {}", result.raw_path.display());
    println!("bronze_path : {}", result.bronze_path.display());
    println!("record_count: {}", result.record_count);

    Ok(())
}
