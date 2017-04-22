use std::io::prelude::*;
use std::io;

use util::nice_num;

// We don't want to spam stdout too much, so only display a byte
// count update when at least the following number of bytes have been
// counted since our last update.
const REPORT_CHUNK_SIZE: u64 = 10_000_000;

pub struct Reporter {
  count: u64,
  last_reported_count: u64,
}

impl Reporter {
  pub fn new() -> Reporter {
    Reporter { count: 0, last_reported_count: 0 }
  }

  pub fn count_bytes(&mut self, count: u64) -> u64 {
    self.count += count;

    if self.last_reported_count == 0 ||
       self.count - self.last_reported_count >= REPORT_CHUNK_SIZE {
      let mut console = io::stdout();
      write!(console, "\rCounted {} bytes.", nice_num(self.count)).unwrap();
      console.flush().unwrap();

      self.last_reported_count = self.count;
    }

    count
  }

  pub fn error_accessing(&mut self, path: &str, e: io::Error) {
    println!("\rError accessing {}: {}.", path, e);

    // Print another byte count message ASAP.
    self.last_reported_count = 0;
  }
}
