extern crate csv;

use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::fs;

use util::nice_num;
use reporter::Reporter;

type DirMap = HashMap<String, u64>;

pub fn get_dir_size(map: &mut DirMap,
                    writer: &mut csv::Writer<fs::File>,
                    path: &Path,
                    reporter: &mut Reporter) -> u64 {
  let path_str = path.to_str().unwrap();

  match map.get(path_str) {
    Some(size) => return reporter.count_bytes(*size),
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
          total += get_dir_size(map, writer, &subpath, reporter);
        } else {
          total += reporter.count_bytes(metadata.len());
        }
      }
    },
    Err(e) => {
      reporter.error_accessing(path_str, e);
    },
  }

  map.insert(String::from(path_str), total);
  writer.encode((path_str, total)).unwrap();

  total
}

pub fn load_or_create_csvfile(csvfile: &Path, map: &mut DirMap) {
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

pub fn create_biggest_csvfile(csvfile: &Path, map: &DirMap, big_size: u64) {
  let mut vec = Vec::new();

  for (path_str, &path_size) in map.iter() {
    if path_size >= big_size {
      vec.push((path_str, path_size));
    }
  }

  vec.sort_by_key(|&(_, size)| size );

  println!("{} directories are bigger than {} bytes.",
           nice_num(vec.len()), nice_num(big_size));

  let mut csv_writer = csv::Writer::from_file(csvfile).unwrap();

  csv_writer.encode(("Directory", "Size")).unwrap();

  for &(path_str, path_size) in vec.iter() {
    csv_writer.encode((path_str, path_size)).unwrap();
  }

  println!("Wrote {}.", csvfile.to_str().unwrap());
}
