extern crate dir_sizer;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::collections::HashMap;

use dir_sizer::reporter::Reporter;
use dir_sizer::dir_map;

const CSV_FILE: &'static str = "dirs.csv";
const BIGGEST_CSV_FILE: &'static str = "biggest_dirs.csv";
const BIG_SIZE: u64 = 100_000_000;

fn main() {
  let mut map = HashMap::new();
  let mut root_path = env::current_dir().unwrap();

  if env::args().count() >= 2 {
    root_path = PathBuf::from(env::args().nth(1).unwrap());
  }

  let mut reporter = Reporter::new();

  dir_map::create_csvfile(
    &Path::new(CSV_FILE),
    root_path.as_path(),
    &mut map,
    &mut reporter
  );

  dir_map::create_biggest_csvfile(
    &Path::new(BIGGEST_CSV_FILE),
    &map,
    BIG_SIZE
  );
}
