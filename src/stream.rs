use crate::error::{CsvError, Result};
use std::io::{BufReader, Read};
use std::sync::mpsc;
use std::thread;

/// Streaming CSV reader for memory-efficient processing of large files
pub struct StreamingCsvReader<R: Read + Send> {
    reader: BufReader<R>,
    buffer_size: usize,
}

impl<R: Read + Send> StreamingCsvReader<R> {
    /// Create a new streaming CSV reader with the specified buffer size
    pub fn new(reader: R, buffer_size: usize) -> Self {
        Self {
            reader: BufReader::with_capacity(buffer_size, reader),
            buffer_size,
        }
    }

    /// Process CSV data in chunks to minimize memory usage
    pub fn process_chunks<F>(&mut self, mut processor: F) -> Result<()>
    where
        F: FnMut(Vec<u8>) -> Result<()> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let buffer_size = self.buffer_size;

        // Spawn worker thread for concurrent processing
        let processor_handle = thread::spawn(move || {
            while let Ok(chunk) = rx.recv() {
                if let Err(e) = processor(chunk) {
                    eprintln!("Processing error: {}", e);
                    return Err(e);
                }
            }
            Ok(())
        });

        // Read data in chunks and send to processing thread
        let mut buffer = vec![0u8; buffer_size];
        loop {
            match self.reader.read(&mut buffer) {
                Ok(0) => break, // End of file reached
                Ok(n) => {
                    let chunk = buffer[..n].to_vec();
                    if tx.send(chunk).is_err() {
                        break;
                    }
                }
                Err(e) => return Err(CsvError::Io(e)),
            }
        }

        drop(tx); // Signal end of data to processing thread
        processor_handle.join().map_err(|_| CsvError::Threading("Processor thread panicked".to_string()))?
    }
}

/// Memory-mapped file reader for very large files (macOS only)
#[cfg(target_os = "macos")]
pub struct MemoryMappedReader {
    _mmap: memmap2::Mmap,
    position: usize,
}

#[cfg(target_os = "macos")]
impl MemoryMappedReader {
    /// Create a new memory-mapped reader for the given file
    pub fn new(file: std::fs::File) -> Result<Self> {
        use memmap2::MmapOptions;
        
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| CsvError::Io(e))?
        };

        Ok(Self {
            _mmap: mmap,
            position: 0,
        })
    }

    /// Read a chunk of data from the memory-mapped file
    pub fn read_chunk(&mut self, size: usize) -> Option<&[u8]> {
        if self.position >= self._mmap.len() {
            return None;
        }

        let end = std::cmp::min(self.position + size, self._mmap.len());
        let chunk = &self._mmap[self.position..end];
        self.position = end;
        Some(chunk)
    }
}
