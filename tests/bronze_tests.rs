// tests\bronze_tests.rs
use std::fs;

use rs_foundry::core::types::RunId;
use rs_foundry::jobs::raw_landing::{land_raw_bytes, land_raw_text};
use rs_foundry::pipelines::bronze::ingest_ref::{
    build_bronze_ref_records, run_bronze_ref_pipeline, sample_ref_source_records,
};

#[test]
fn land_raw_text_writes_file_to_expected_location() {
    let temp_dir = tempfile::tempdir().unwrap();
    let run_id = RunId::new();

    let result = land_raw_text(
        temp_dir.path(),
        "ref_example",
        &run_id,
        Some("sample.json"),
        r#"{"hello":"world"}"#,
    )
    .unwrap();

    assert!(result.file_path.exists());
    assert_eq!(result.source_name, "ref_example");
    assert_eq!(result.run_id, run_id.0.to_string());

    let content = fs::read_to_string(&result.file_path).unwrap();
    assert_eq!(content, r#"{"hello":"world"}"#);
}

#[test]
fn land_raw_bytes_writes_binary_payload() {
    let temp_dir = tempfile::tempdir().unwrap();
    let run_id = RunId::new();
    let payload = vec![1_u8, 2_u8, 3_u8, 4_u8];

    let result = land_raw_bytes(
        temp_dir.path(),
        "daily_example",
        &run_id,
        Some("sample.bin"),
        &payload,
    )
    .unwrap();

    assert!(result.file_path.exists());
    assert_eq!(result.bytes_written, payload.len());

    let written = fs::read(&result.file_path).unwrap();
    assert_eq!(written, payload);
}

#[test]
fn raw_landing_uses_default_file_name_when_missing() {
    let temp_dir = tempfile::tempdir().unwrap();
    let run_id = RunId::new();

    let result = land_raw_text(temp_dir.path(), "ref_example", &run_id, None, "abc").unwrap();

    let file_name = result.file_path.file_name().unwrap().to_string_lossy();
    assert_eq!(file_name, "payload.txt");
}

#[test]
fn bronze_ref_record_builder_enriches_source_records() {
    let source_records = sample_ref_source_records();
    let run_id = RunId::new();

    let bronze_records = build_bronze_ref_records(
        &source_records,
        &run_id,
        "ref_example",
        Some("ref_source.json".to_string()),
    );

    assert_eq!(bronze_records.len(), 3);
    assert_eq!(bronze_records[0].ref_id, "R001");
    assert_eq!(bronze_records[0].source_name, "ref_example");
    assert_eq!(bronze_records[0].source_file.as_deref(), Some("ref_source.json"));
    assert_eq!(bronze_records[0].run_id, run_id.0.to_string());
}

#[test]
fn bronze_ref_pipeline_writes_raw_and_bronze_outputs() {
    let result = run_bronze_ref_pipeline().unwrap();

    assert_eq!(result.source_name, "ref_example");
    assert_eq!(result.record_count, 3);
    assert!(result.raw_path.exists());
    assert!(result.bronze_path.exists());

    let raw_content = fs::read_to_string(&result.raw_path).unwrap();
    assert!(raw_content.contains("R001"));

    let bronze_content = fs::read_to_string(&result.bronze_path).unwrap();
    assert!(bronze_content.contains("source_name"));
    assert!(bronze_content.contains("run_id"));

    assert!(result.quality_report.passed);
    assert_eq!(result.quality_report.errors.len(), 0);

}

use rs_foundry::pipelines::bronze::ingest_daily::{
    build_bronze_daily_records, run_bronze_daily_pipeline, sample_daily_source_records,
};

#[test]
fn bronze_daily_record_builder_enriches_source_records() {
    let source_records = sample_daily_source_records();
    let run_id = RunId::new();

    let bronze_records = build_bronze_daily_records(
        &source_records,
        &run_id,
        "daily_example",
        Some("daily_source.json".to_string()),
    );

    assert_eq!(bronze_records.len(), 3);
    assert_eq!(bronze_records[0].event_id, "E001");
    assert_eq!(bronze_records[0].ref_id, "R001");
    assert_eq!(bronze_records[0].source_name, "daily_example");
    assert_eq!(bronze_records[0].run_id, run_id.0.to_string());
}

#[test]
fn bronze_daily_pipeline_writes_raw_and_bronze_outputs() {
    let result = run_bronze_daily_pipeline().unwrap();

    assert_eq!(result.source_name, "daily_example");
    assert_eq!(result.record_count, 3);
    assert!(result.raw_path.exists());
    assert!(result.bronze_path.exists());

    let raw_content = fs::read_to_string(&result.raw_path).unwrap();
    assert!(raw_content.contains("E001"));

    let bronze_content = fs::read_to_string(&result.bronze_path).unwrap();
    assert!(bronze_content.contains("metric_value"));
    assert!(bronze_content.contains("source_name"));
    assert!(bronze_content.contains("run_id"));

    assert!(result.quality_report.passed);
    assert_eq!(result.quality_report.errors.len(), 0);

}
