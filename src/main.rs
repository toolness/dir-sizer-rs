#[macro_use(crate_name, crate_version, crate_authors)]
extern crate clap;
extern crate dir_sizer;

use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Path;
use std::collections::HashMap;
use clap::{App, Arg};

use dir_sizer::reporter::Reporter;
use dir_sizer::dir_map;

const CSV_FILE: &'static str = "dirs.csv";
const BIGGEST_CSV_FILE: &'static str = "biggest_dirs.csv";
const BIG_SIZE: u64 = 100_000_000;

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about("Generates information about the directories taking up \
           lots of space on your system.")
    .arg(Arg::with_name("PATH")
         .help("The directory to profile (defaults to current working dir)")
         .index(1))
    .get_matches();

  let mut map = HashMap::new();
  let mut root_path = env::current_dir().unwrap();

  if let Some(path) = matches.value_of("PATH") {
    root_path = PathBuf::from(path);
    if !root_path.exists() {
      println!("The path '{}' does not exist.", path);
      process::exit(1);
    }
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
    root_path.as_path(),
    &map,
    BIG_SIZE
  );
}
