//! High-performance CSV parser for large files (up to 100GB)

pub mod config;
pub mod error;
pub mod processor;
pub mod stream;
pub mod stats;

pub use error::{CsvError, Result};
