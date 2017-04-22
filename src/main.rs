extern crate csv;

use std::io::prelude::*;
use std::io;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::path::Path;
use std::vec::Vec;
use std::fs;
use std::collections::HashMap;

const CSV_FILE: &'static str = "dirs.csv";
const BIGGEST_CSV_FILE: &'static str = "biggest_dirs.csv";
const BIG_SIZE: &'static u64 = &100_000_000;
const MED_SIZE: &'static u64 = &10_000_000;

type DirMap = HashMap<String, u64>;

struct AccumulatedBytes {
  count: u64,
  last_reported_count: u64,
}

impl AccumulatedBytes {
  fn new() -> AccumulatedBytes {
    AccumulatedBytes { count: 0, last_reported_count: 0 }
  }

  fn add(&mut self, count: u64) -> u64 {
    self.count += count;

    if self.last_reported_count == 0 ||
       self.count - self.last_reported_count >= *MED_SIZE {
      let mut console = io::stdout();
      write!(console, "\rCounted {} bytes.", nice_num(self.count)).unwrap();
      console.flush().unwrap();

      self.last_reported_count = self.count;
    }

    count
  }

  fn report_access_error(&mut self, path: &str, e: io::Error) {
    println!("\rError accessing {}: {}.", path, e);
    self.last_reported_count = 0;
  }
}

fn nice_num<T: Display>(number: T) -> String {
  let mut result = String::new();
  let num_str = format!("{}", number);
  let num_digits = num_str.len();
  let mut comma_counter = 0;

  if num_str.len() > 3 {
    comma_counter = ((num_digits / 3 + 1) * 3 - num_digits) % 3;
  }

  for (i, c) in num_str.chars().enumerate() {
    let is_last_digit = i == num_digits - 1;
    result.push(c);
    comma_counter += 1;
    if comma_counter == 3 && !is_last_digit {
      result.push(',');
      comma_counter = 0;
    }
  }

  return result;
}

#[test]
fn nice_num_works() {
  assert_eq!(nice_num(1), "1");
  assert_eq!(nice_num(15), "15");
  assert_eq!(nice_num(153), "153");
  assert_eq!(nice_num(1534), "1,534");
  assert_eq!(nice_num(51534), "51,534");
  assert_eq!(nice_num(651534), "651,534");
  assert_eq!(nice_num(7651534), "7,651,534");
  assert_eq!(nice_num(87651534), "87,651,534");
  assert_eq!(nice_num(987651534), "987,651,534");
  assert_eq!(nice_num(1987651534), "1,987,651,534");
}

fn get_dir_size(map: &mut DirMap,
                writer: &mut csv::Writer<fs::File>,
                path: &Path,
                accumulator: &mut AccumulatedBytes) -> u64 {
  let path_str = path.to_str().unwrap();

  match map.get(path_str) {
    Some(size) => return accumulator.add(*size),
    None => {},
  }

  let mut total = 0;

  match fs::read_dir(path) {
    Ok(read_dir) => {
      for wrapped_entry in read_dir {
        let entry = wrapped_entry.unwrap();
        let subpath = entry.path();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
          total += get_dir_size(map, writer, &subpath, accumulator);
        } else {
          total += accumulator.add(metadata.len());
        }
      }
    },
    Err(e) => {
      accumulator.report_access_error(path_str, e);
    },
  }

  map.insert(String::from(path_str), total);
  writer.encode((path_str, total)).unwrap();

  total
}

fn load_or_create_csvfile(csvfile: &Path, map: &mut DirMap) {
  if csvfile.exists() {
    println!("Loading {}...", csvfile.to_str().unwrap());
    let mut reader = csv::Reader::from_file(csvfile).unwrap();
    for record in reader.decode() {
      let (path_str, size): (String, u64) = record.unwrap();
      map.insert(path_str, size);
    }
    println!("Loaded {} record(s).", nice_num(map.len()));
  } else {
    let mut file = fs::File::create(csvfile).unwrap();
    file.write_all(b"Directory,Size\n").unwrap();
  }
}

fn create_biggest_csvfile(csvfile: &Path, map: &DirMap, big_size: &u64) {
  let mut vec = Vec::new();

  for (path_str, path_size) in map.iter() {
    if path_size >= big_size {
      vec.push((path_str, path_size));
    }
  }

  vec.sort_by_key(|&(_, size)| size );

  println!("{} directories are bigger than {} bytes.",
           nice_num(vec.len()), nice_num(*big_size));

  let mut csv_writer = csv::Writer::from_file(csvfile).unwrap();

  csv_writer.encode(("Directory", "Size")).unwrap();

  for &(path_str, path_size) in vec.iter() {
    csv_writer.encode((path_str, path_size)).unwrap();
  }

  println!("Wrote {}.", csvfile.to_str().unwrap());
}

fn main() {
  let mut map = HashMap::new();
  let csvfile = Path::new(CSV_FILE);

  load_or_create_csvfile(&csvfile, &mut map);

  let mut root_path = env::current_dir().unwrap();

  if env::args().count() >= 2 {
    root_path = PathBuf::from(env::args().nth(1).unwrap());
  }

  let file = fs::OpenOptions::new().append(true).open(csvfile).unwrap();
  let mut csv_writer = csv::Writer::from_writer(file);
  let mut accumulator = AccumulatedBytes::new();
  let size = get_dir_size(&mut map, &mut csv_writer, root_path.as_path(),
                          &mut accumulator);

  println!("\nTotal size of {} is {} bytes.",
           root_path.to_str().unwrap(), nice_num(size));

  create_biggest_csvfile(&Path::new(BIGGEST_CSV_FILE), &map, BIG_SIZE);
}
