use thiserror::Error;

/// Result type alias for CSV parser operations
pub type Result<T> = std::result::Result<T, CsvError>;

/// Comprehensive error types for CSV parser operations
#[derive(Error, Debug)]
pub enum CsvError {
    /// I/O related errors (file operations, network, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV parsing and formatting errors
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),

    /// Configuration and setup errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// General processing errors
    #[error("Processing error: {0}")]
    Processing(String),

    /// Field selection and validation errors
    #[error("Field selection error: {0}")]
    FieldSelection(String),

    /// Threading and concurrency errors
    #[error("Threading error: {0}")]
    Threading(String),
}
