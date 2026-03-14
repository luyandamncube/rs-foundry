// src\io\files.rs
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::core::errors::RsFoundryError;
use crate::core::types::RunId;

pub fn ensure_dir(path: &Path) -> Result<(), RsFoundryError> {
    fs::create_dir_all(path)
        .map_err(|e| RsFoundryError::Io(format!("failed to create directory {:?}: {e}", path)))
}

pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<(), RsFoundryError> {
    if let Some(parent) = path.parent() {
        ensure_dir(parent)?;
    }

    fs::write(path, bytes)
        .map_err(|e| RsFoundryError::Io(format!("failed to write file {:?}: {e}", path)))
}

pub fn write_text(path: &Path, content: &str) -> Result<(), RsFoundryError> {
    write_bytes(path, content.as_bytes())
}

pub fn file_name_or_default(file_name: Option<&str>, default_name: &str) -> String {
    file_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(default_name)
        .to_string()
}

pub fn dated_partition() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

pub fn raw_run_dir(
    data_root: &Path,
    source_name: &str,
    run_id: &RunId,
    partition_date: &str,
) -> PathBuf {
    data_root
        .join("raw")
        .join(source_name)
        .join(format!("load_date={partition_date}"))
        .join(format!("run_id={}", run_id.0))
}

pub fn raw_file_path(
    data_root: &Path,
    source_name: &str,
    run_id: &RunId,
    partition_date: &str,
    file_name: &str,
) -> PathBuf {
    raw_run_dir(data_root, source_name, run_id, partition_date).join(file_name)
}
