// src\config\paths.rs
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DataPaths {
    root: PathBuf,
}

impl DataPaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn raw(&self) -> PathBuf {
        self.root.join("raw")
    }

    pub fn bronze(&self) -> PathBuf {
        self.root.join("bronze")
    }

    pub fn silver(&self) -> PathBuf {
        self.root.join("silver")
    }

    pub fn gold(&self) -> PathBuf {
        self.root.join("gold")
    }

    pub fn checkpoints(&self) -> PathBuf {
        self.root.join("checkpoints")
    }

    pub fn runs(&self) -> PathBuf {
        self.root.join("runs")
    }

    pub fn ensure_layout(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.raw())?;
        std::fs::create_dir_all(self.bronze())?;
        std::fs::create_dir_all(self.silver())?;
        std::fs::create_dir_all(self.gold())?;
        std::fs::create_dir_all(self.checkpoints())?;
        std::fs::create_dir_all(self.runs())?;
        Ok(())
    }
}
