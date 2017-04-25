#[macro_use(crate_name, crate_version, crate_authors)]
extern crate clap;
extern crate dir_sizer;

use std::env;
use std::process;
use std::path::PathBuf;
use clap::{App, Arg};

use dir_sizer::reporter::Reporter;
use dir_sizer::dir_mapper::DirMapper;
use dir_sizer::util::strip_commas;

const DEFAULT_CSV_FILE: &'static str = "dirs.csv";
const DEFAULT_BIG_CSV_FILE: &'static str = "big_dirs.csv";
const DEFAULT_BIG_SIZE: &'static str = "100,000,000";

fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .about("Generates information about the directories taking up \
           lots of space on your system.")
    .arg(Arg::with_name("csv_file")
         .short("c")
         .long("csv-file")
         .help("Name of CSV file in which to store/cache all directory info")
         .value_name("FILE")
         .default_value(DEFAULT_CSV_FILE)
         .takes_value(true))
    .arg(Arg::with_name("big_csv_file")
         .short("b")
         .long("big-csv-file")
         .help("Name of CSV file in which to list big directories")
         .value_name("FILE")
         .default_value(DEFAULT_BIG_CSV_FILE)
         .takes_value(true))
    .arg(Arg::with_name("big_size")
         .short("s")
         .long("big-size")
         .help("A dir must be this big (in bytes) to be considered 'big'")
         .value_name("SIZE")
         .default_value(DEFAULT_BIG_SIZE)
         .takes_value(true))
    .arg(Arg::with_name("PATH")
         .help("The directory to profile (defaults to current working dir)")
         .index(1))
    .get_matches();

  let big_size = strip_commas(matches.value_of("big_size").unwrap())
    .parse::<u64>().unwrap();
  let mut root_path = env::current_dir().unwrap();

  if let Some(path) = matches.value_of("PATH") {
    root_path = PathBuf::from(path);
    if !root_path.exists() {
      println!("The path '{}' does not exist.", path);
      process::exit(1);
    }
  }

  let mut reporter = Reporter::new();
  let mut mapper = DirMapper::new(root_path);

  mapper.create_csvfile(
    matches.value_of("csv_file").unwrap(),
    &mut reporter
  );

  mapper.create_big_csvfile(
    matches.value_of("big_csv_file").unwrap(),
    big_size
  );
}
