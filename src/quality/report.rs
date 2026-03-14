// src\quality\report.rs
#[derive(Debug, Clone)]
pub struct QualityReport {
    pub passed: bool,
    pub messages: Vec<String>,
}

impl Default for QualityReport {
    fn default() -> Self {
        Self {
            passed: true,
            messages: Vec::new(),
        }
    }
}
