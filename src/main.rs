//! High-performance CSV parser for large files (up to 100GB)

use anyhow::Result;
use clap::Parser;
use csvparser::config::Config;
use csvparser::processor::CsvProcessor;
use std::process;
use tracing::info;

/// Command-line interface for the CSV parser
#[derive(Parser)]
#[command(
    name = "csvparser",
    version = "0.1.0",
    about = "High-performance CSV parser for large files (up to 100GB)",
    long_about = "A memory-efficient CSV parser designed to handle very large files (up to 100GB) with minimal memory usage and maximum performance."
)]
struct Cli {
    /// Input CSV file (default: stdin)
    input: Option<String>,
    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
    /// Select specific fields (comma-separated, 1-based indexing)
    #[arg(short, long, value_delimiter = ',')]
    fields: Option<Vec<usize>>,
    /// Buffer size in bytes (default: 64KB)
    #[arg(long, default_value = "65536")]
    buffer_size: usize,
    /// Number of worker threads (default: auto-detect)
    #[arg(short, long)]
    threads: Option<usize>,
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    /// Show processing statistics
    #[arg(long)]
    stats: bool,
}

/// Main entry point for the CSV parser application
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging with appropriate level
    let log_level = if cli.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .with_writer(std::io::stderr)
        .init();

    // Create configuration from CLI arguments
    let config = Config {
        input: cli.input,
        output: cli.output,
        fields: cli.fields,
        buffer_size: cli.buffer_size,
        threads: cli.threads,
        stats: cli.stats,
    };

    // Run the CSV processing
    if let Err(e) = run(config).await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Execute the CSV processing pipeline
async fn run(config: Config) -> Result<()> {
    info!("Starting CSV parser with config: {:?}", config);
    
    let mut processor = CsvProcessor::new(config)?;
    processor.process().await?;
    
    info!("CSV processing completed successfully");
    Ok(())
}
