// src\bin\silver_conformed_build.rs
fn main() -> anyhow::Result<()> {
    let ref_bronze_run_id = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("missing ref_bronze_run_id argument"))?;

    let daily_bronze_run_id = std::env::args()
        .nth(2)
        .ok_or_else(|| anyhow::anyhow!("missing daily_bronze_run_id argument"))?;

    let result = rs_foundry::jobs::silver_conformed_job::execute(
        &ref_bronze_run_id,
        &daily_bronze_run_id,
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    println!("silver_conformed_build complete");
    println!("ref_bronze_run_id  : {}", result.ref_bronze_run_id);
    println!("daily_bronze_run_id: {}", result.daily_bronze_run_id);
    println!("conformed_path     : {}", result.conformed_path.display());
    println!("record_count       : {}", result.record_count);

    Ok(())
}