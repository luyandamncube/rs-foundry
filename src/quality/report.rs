// src\quality\report.rs
#[derive(Debug, Clone, Default)]
pub struct QualityReport {
    pub passed: bool,
    pub row_count: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl QualityReport {
    pub fn new(row_count: usize) -> Self {
        Self {
            passed: true,
            row_count,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn push_error(&mut self, message: impl Into<String>) {
        self.passed = false;
        self.errors.push(message.into());
    }

    pub fn push_warning(&mut self, message: impl Into<String>) {
        self.warnings.push(message.into());
    }
}
