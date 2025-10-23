use crate::config::Config;
use crate::error::{CsvError, Result};
use crate::stats::ProcessingStats;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info, warn};

/// Main CSV processor that handles streaming processing of large CSV files
pub struct CsvProcessor {
    config: Config,
    stats: Arc<ProcessingStats>,
}

impl CsvProcessor {
    /// Create a new CSV processor with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let stats = Arc::new(ProcessingStats::new());
        Ok(Self { config, stats })
    }

    /// Process the CSV file with streaming approach for memory efficiency
    pub async fn process(&mut self) -> Result<()> {
        let start_time = Instant::now();
        info!("Starting CSV processing");

        // Open input source (file or stdin)
        let input_reader = self.open_input()?;
        // Open output destination (file or stdout)
        let output_writer = self.open_output()?;
        // Process CSV with streaming approach
        self.process_streaming(input_reader, output_writer).await?;

        let duration = start_time.elapsed();
        info!("Processing completed in {:?}", duration);

        // Print statistics if requested
        if self.config.stats {
            self.print_stats(duration);
        }

        Ok(())
    }

    /// Open the input source (file or stdin) with buffering
    fn open_input(&self) -> Result<Box<dyn io::Read + Send>> {
        match &self.config.input {
            Some(path) => {
                let file = File::open(path)
                    .map_err(|e| CsvError::Io(e))?;
                Ok(Box::new(BufReader::with_capacity(
                    self.config.buffer_size,
                    file,
                )))
            }
            None => Ok(Box::new(io::stdin())),
        }
    }

    /// Open the output destination (file or stdout) with buffering
    fn open_output(&self) -> Result<Box<dyn Write + Send>> {
        match &self.config.output {
            Some(path) => {
                let file = File::create(path)
                    .map_err(|e| CsvError::Io(e))?;
                Ok(Box::new(BufWriter::with_capacity(
                    self.config.buffer_size,
                    file,
                )))
            }
            None => Ok(Box::new(io::stdout())),
        }
    }

    /// Process CSV data in streaming fashion for memory efficiency
    async fn process_streaming(
        &mut self,
        mut input_reader: Box<dyn io::Read + Send>,
        mut output_writer: Box<dyn Write + Send>,
    ) -> Result<()> {
        // Create CSV reader with optimized settings for large files
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .buffer_capacity(self.config.buffer_size)
            .flexible(true)
            .from_reader(&mut input_reader);

        // Extract and process headers
        let headers = reader.headers()?.clone();
        debug!("Headers: {:?}", headers);

        // Write headers to output with field selection if specified
        if let Some(field_indices) = self.config.field_indices() {
            let selected_headers: Vec<_> = field_indices
                .iter()
                .filter_map(|&i| headers.get(i))
                .collect();
            output_writer.write_all(&format!("{}\n", selected_headers.join(",")).as_bytes())?;
        } else {
            output_writer.write_all(&format!("{}\n", headers.iter().collect::<Vec<_>>().join(",")).as_bytes())?;
        }

        // Process records in streaming fashion to minimize memory usage
        let mut record_count = 0u64;

        for result in reader.records() {
            match result {
                Ok(record) => {
                    // Apply field selection if specified
                    if let Some(field_indices) = self.config.field_indices() {
                        let selected_fields: Vec<_> = field_indices
                            .iter()
                            .filter_map(|&i| record.get(i))
                            .collect();
                        output_writer.write_all(&format!("{}\n", selected_fields.join(",")).as_bytes())?;
                    } else {
                        output_writer.write_all(&format!("{}\n", record.iter().collect::<Vec<_>>().join(",")).as_bytes())?;
                    }
                    
                    record_count += 1;
                    self.stats.update_records_processed(record_count);

                    // Progress reporting for large files
                    if record_count % 100000 == 0 {
                        debug!("Processed {} records", record_count);
                    }
                }
                Err(e) => {
                    warn!("CSV parsing error: {}", e);
                    continue;
                }
            }
        }

        // Ensure all data is written to output
        output_writer.flush()?;
        info!("Total records processed: {}", record_count);
        Ok(())
    }

    /// Print processing statistics to stderr
    fn print_stats(&self, duration: std::time::Duration) {
        let stats = &*self.stats;
        eprintln!("\n=== Processing Statistics ===");
        eprintln!("Records processed: {}", stats.records_processed());
        eprintln!("Processing time: {:?}", duration);
        eprintln!("Records per second: {:.2}", 
            stats.records_processed() as f64 / duration.as_secs_f64());
        eprintln!("Memory usage: {} MB", 
            self.get_memory_usage_mb());
    }

    /// Calculate estimated memory usage in MB
    fn get_memory_usage_mb(&self) -> f64 {
        let buffer_size_mb = self.config.buffer_size as f64 / (1024.0 * 1024.0);
        buffer_size_mb * 2.0
    }
}
