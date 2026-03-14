// src\core\errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RsFoundryError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("not implemented: {0}")]
    NotImplemented(String),
}
