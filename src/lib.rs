//! Provide progress callbacks for types which implement `io::Read` or `io::Write`.
//! 
//! ## Examples
//! 
//! ### Reader
//! 
//! ```rust,no_run
//! extern crate progress_streams;
//! 
//! use progress_streams::ProgressReader;
//! use std::fs::File;
//! use std::io::Read;
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::thread;
//! use std::time::Duration;
//! 
//! fn main() {
//!     let total = Arc::new(AtomicUsize::new(0));
//!     let mut file = File::open("/dev/urandom").unwrap();
//!     let mut reader = ProgressReader::new(&mut file, |progress: usize| {
//!         total.fetch_add(progress, Ordering::SeqCst);
//!     });
//! 
//!     {
//!         let total = total.clone();
//!         thread::spawn(move || {
//!             loop {
//!                 println!("Read {} KiB", total.load(Ordering::SeqCst) / 1024);
//!                 thread::sleep(Duration::from_millis(16));
//!             }
//!         });
//!     }
//! 
//!     let mut buffer = [0u8; 8192];
//!     while total.load(Ordering::SeqCst) < 100 * 1024 * 1024 {
//!         reader.read(&mut buffer).unwrap();
//!     }
//! }
//! ```
//! 
//! ### Writer
//! 
//! ```rust,no_run
//! extern crate progress_streams;
//! 
//! use progress_streams::ProgressWriter;
//! use std::io::{Cursor, Write};
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::thread;
//! use std::time::Duration;
//! 
//! fn main() {
//!     let total = Arc::new(AtomicUsize::new(0));
//!     let mut file = Cursor::new(Vec::new());
//!     let mut writer = ProgressWriter::new(&mut file, |progress: usize| {
//!         total.fetch_add(progress, Ordering::SeqCst);
//!     });
//! 
//!     {
//!         let total = total.clone();
//!         thread::spawn(move || {
//!             loop {
//!                 println!("Written {} Kib", total.load(Ordering::SeqCst) / 1024);
//!                 thread::sleep(Duration::from_millis(16));
//!             }
//!         });
//!     }
//! 
//!     let buffer = [0u8; 8192];
//!     while total.load(Ordering::SeqCst) < 1000 * 1024 * 1024 {
//!         writer.write(&buffer).unwrap();
//!     }
//! }
//! ```

use std::io::{self, Read, Write};

/// Callback-based progress-monitoring writer.
pub struct ProgressWriter<W: Write, C: FnMut(usize)> {
    writer: W,
    callback: C
}

impl<W: Write, C: FnMut(usize)> ProgressWriter<W, C> {
    pub fn new(writer: W, callback: C) -> Self {
        Self { writer, callback }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write, C: FnMut(usize)> Write for ProgressWriter<W, C> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.writer.write(buf)?;
        (self.callback)(written);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Callback-based progress-monitoring reader.
pub struct ProgressReader<R: Read, C: FnMut(usize)> {
    reader: R,
    callback: C
}

impl<R: Read, C: FnMut(usize)> ProgressReader<R, C> {
    pub fn new(reader: R, callback: C) -> Self {
        Self { reader, callback }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R: Read, C: FnMut(usize)> Read for ProgressReader<R, C> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read = self.reader.read(buf)?;
        (self.callback)(read);
        Ok(read)
    }
}
