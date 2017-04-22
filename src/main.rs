extern crate csv;
extern crate dir_sizer;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

use dir_sizer::util::nice_num;
use dir_sizer::reporter::Reporter;
use dir_sizer::dir_map;

const CSV_FILE: &'static str = "dirs.csv";
const BIGGEST_CSV_FILE: &'static str = "biggest_dirs.csv";
const BIG_SIZE: u64 = 100_000_000;

fn main() {
  let mut map = HashMap::new();
  let csvfile = Path::new(CSV_FILE);

  dir_map::load_or_create_csvfile(&csvfile, &mut map);

  let mut root_path = env::current_dir().unwrap();

  if env::args().count() >= 2 {
    root_path = PathBuf::from(env::args().nth(1).unwrap());
  }

  let file = fs::OpenOptions::new().append(true).open(csvfile).unwrap();
  let mut csv_writer = csv::Writer::from_writer(file);
  let mut reporter = Reporter::new();
  let size = dir_map::get_dir_size(
    &mut map,
    &mut csv_writer,
    root_path.as_path(),
    &mut reporter
  );

  println!("\nTotal size of {} is {} bytes.",
           root_path.to_str().unwrap(), nice_num(size));

  dir_map::create_biggest_csvfile(
    &Path::new(BIGGEST_CSV_FILE),
    &map,
    BIG_SIZE
  );
}
