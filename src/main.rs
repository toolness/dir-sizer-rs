extern crate csv;

use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

const ROOT_DIR: &'static str = "C:\\utils";
const CSV_FILE: &'static str = "dirs.csv";

fn get_dir_size(map: &mut HashMap<String, u64>,
                writer: &mut csv::Writer<fs::File>,
                path: &Path) -> u64 {
  let path_str = path.to_str().unwrap();

  match map.get(path_str) {
    Some(size) => return *size,
    None => {},
  }

  let mut total = 0;

  println!("Calculating size of {}.", path_str);

  for wrapped_entry in fs::read_dir(path).unwrap() {
    let entry = wrapped_entry.unwrap();
    let subpath = entry.path();
    let metadata = entry.metadata().unwrap();
    if metadata.is_dir() {
      total += get_dir_size(map, writer, &subpath);
    } else {
      total += metadata.len();
    }
  }

  map.insert(String::from(path_str), total);
  writer.encode((path_str, total)).unwrap();

  total
}

fn main() {
  let mut map = HashMap::new();
  let csvfile = Path::new(CSV_FILE);

  if csvfile.exists() {
    let mut reader = csv::Reader::from_file(csvfile).unwrap();
    for record in reader.decode() {
      let (path_str, size): (String, u64) = record.unwrap();
      map.insert(path_str, size);
    }
  } else {
    let mut file = fs::File::create(csvfile).unwrap();
    file.write_all(b"Directory,Size\n").unwrap();
  }

  let root_path = Path::new(ROOT_DIR);
  let file = fs::OpenOptions::new().append(true).open(csvfile).unwrap();
  let mut writer = csv::Writer::from_writer(file);
  let size = get_dir_size(&mut map, &mut writer, root_path);

  println!("Total size of {:?} is {}.", root_path, size);
}
