use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration settings for the CSV parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Input file path (None for stdin)
    pub input: Option<String>,
    /// Output file path (None for stdout)
    pub output: Option<String>,
    /// Field indices to select (1-based indexing)
    pub fields: Option<Vec<usize>>,
    /// Buffer size in bytes for I/O operations
    pub buffer_size: usize,
    /// Number of worker threads (None for auto-detection)
    pub threads: Option<usize>,
    /// Whether to show processing statistics
    pub stats: bool,
}

impl Config {
    /// Get the input file path as a PathBuf
    pub fn input_path(&self) -> Option<PathBuf> {
        self.input.as_ref().map(PathBuf::from)
    }

    /// Get the output file path as a PathBuf
    pub fn output_path(&self) -> Option<PathBuf> {
        self.output.as_ref().map(PathBuf::from)
    }

    /// Check if field selection is enabled
    pub fn should_select_fields(&self) -> bool {
        self.fields.is_some()
    }

    /// Get field indices converted to 0-based indexing
    pub fn field_indices(&self) -> Option<Vec<usize>> {
        self.fields.as_ref().map(|fields| {
            fields.iter().map(|&f| f.saturating_sub(1)).collect()
        })
    }
}
