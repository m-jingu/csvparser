use std::sync::atomic::{AtomicU64, Ordering};

/// Thread-safe statistics tracker for CSV processing metrics
#[derive(Debug)]
pub struct ProcessingStats {
    records_processed: AtomicU64,
    bytes_processed: AtomicU64,
    start_time: std::time::Instant,
}

impl ProcessingStats {
    /// Create a new statistics tracker with initial values
    pub fn new() -> Self {
        Self {
            records_processed: AtomicU64::new(0),
            bytes_processed: AtomicU64::new(0),
            start_time: std::time::Instant::now(),
        }
    }

    /// Update the total number of records processed (absolute value)
    pub fn update_records_processed(&self, count: u64) {
        self.records_processed.store(count, Ordering::Relaxed);
    }

    /// Add to the number of records processed (incremental)
    pub fn add_records_processed(&self, count: u64) {
        self.records_processed.fetch_add(count, Ordering::Relaxed);
    }

    /// Update the total number of bytes processed (absolute value)
    pub fn update_bytes_processed(&self, bytes: u64) {
        self.bytes_processed.store(bytes, Ordering::Relaxed);
    }

    /// Add to the number of bytes processed (incremental)
    pub fn add_bytes_processed(&self, bytes: u64) {
        self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Get the current number of records processed
    pub fn records_processed(&self) -> u64 {
        self.records_processed.load(Ordering::Relaxed)
    }

    /// Get the current number of bytes processed
    pub fn bytes_processed(&self) -> u64 {
        self.bytes_processed.load(Ordering::Relaxed)
    }

    /// Get the elapsed time since statistics tracking started
    pub fn elapsed_time(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Calculate the current processing rate in records per second
    pub fn records_per_second(&self) -> f64 {
        let elapsed = self.elapsed_time().as_secs_f64();
        if elapsed > 0.0 {
            self.records_processed() as f64 / elapsed
        } else {
            0.0
        }
    }

    /// Calculate the current processing rate in bytes per second
    pub fn bytes_per_second(&self) -> f64 {
        let elapsed = self.elapsed_time().as_secs_f64();
        if elapsed > 0.0 {
            self.bytes_processed() as f64 / elapsed
        } else {
            0.0
        }
    }
}
