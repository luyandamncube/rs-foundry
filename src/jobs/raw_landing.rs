// src\jobs\raw_landing.rs
use std::path::{Path, PathBuf};

use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;
use crate::io::files::{dated_partition, file_name_or_default, raw_file_path, write_bytes, write_text};

#[derive(Debug, Clone)]
pub struct RawLandingResult {
    pub source_name: String,
    pub run_id: String,
    pub partition_date: String,
    pub file_path: PathBuf,
    pub bytes_written: usize,
}

pub fn land_raw_bytes(
    data_root: &Path,
    source_name: &str,
    run_id: &RunId,
    file_name: Option<&str>,
    bytes: &[u8],
) -> Result<RawLandingResult, RsFoundryError> {
    let partition_date = dated_partition();
    let file_name = file_name_or_default(file_name, "payload.bin");

    let path = raw_file_path(data_root, source_name, run_id, &partition_date, &file_name);
    write_bytes(&path, bytes)?;

    Ok(RawLandingResult {
        source_name: source_name.to_string(),
        run_id: run_id.0.to_string(),
        partition_date,
        file_path: path,
        bytes_written: bytes.len(),
    })
}

pub fn land_raw_text(
    data_root: &Path,
    source_name: &str,
    run_id: &RunId,
    file_name: Option<&str>,
    content: &str,
) -> Result<RawLandingResult, RsFoundryError> {
    let partition_date = dated_partition();
    let file_name = file_name_or_default(file_name, "payload.txt");

    let path = raw_file_path(data_root, source_name, run_id, &partition_date, &file_name);
    write_text(&path, content)?;

    Ok(RawLandingResult {
        source_name: source_name.to_string(),
        run_id: run_id.0.to_string(),
        partition_date,
        file_path: path,
        bytes_written: content.len(),
    })
}
