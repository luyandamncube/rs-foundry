// src\io\files.rs
use std::fs;
use std::path::Path;

pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(path)
}
