extern crate csv;

use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::fs;

use util::WithCommas;
use reporter::Reporter;

type DirMap = HashMap<String, u64>;

pub struct DirMapper<'a> {
  sizes: HashMap<String, u64>,
  path: &'a Path
}

impl<'a> DirMapper<'a> {
  pub fn new(path: &Path) -> DirMapper {
    DirMapper { sizes: HashMap::new(), path: path }
  }

  pub fn create_csvfile(&mut self, csvfile: &Path,
                        mut reporter: &mut Reporter) {
    create_csvfile(csvfile, self.path, &mut self.sizes, reporter)
  }

  pub fn create_big_csvfile(&mut self, csvfile: &Path, big_size: u64) {
    create_big_csvfile(csvfile, self.path, &mut self.sizes, big_size)
  }
}

fn get_dir_size(map: &mut DirMap,
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

fn load_or_create_csvfile(csvfile: &Path, map: &mut DirMap) {
  if csvfile.exists() {
    println!("Loading {}...", csvfile.to_str().unwrap());
    let mut reader = csv::Reader::from_file(csvfile).unwrap();
    for record in reader.decode() {
      let (path_str, size): (String, u64) = record.unwrap();
      map.insert(path_str, size);
    }
    println!("Loaded {} record(s).", map.len().with_commas());
  } else {
    let mut file = fs::File::create(csvfile).unwrap();
    file.write_all(b"Directory,Size\n").unwrap();
  }
}

fn create_csvfile(csvfile: &Path,
                  root_path: &Path,
                  mut map: &mut DirMap,
                  mut reporter: &mut Reporter) {
  load_or_create_csvfile(&csvfile, &mut map);

  let file = fs::OpenOptions::new().append(true).open(csvfile).unwrap();
  let mut csv_writer = csv::Writer::from_writer(file);
  let size = get_dir_size(
    &mut map,
    &mut csv_writer,
    root_path,
    &mut reporter
  );

  println!("\nTotal size of {} is {} bytes.",
           root_path.to_str().unwrap(), size.with_commas());
}

fn create_big_csvfile(csvfile: &Path,
                      root_path: &Path,
                      map: &DirMap,
                      big_size: u64) {
  let mut vec = Vec::new();
  let root_path_str = root_path.to_str().unwrap();

  for (path_str, &path_size) in map.iter() {
    if path_size >= big_size && path_str.starts_with(root_path_str) {
      vec.push((path_str, path_size));
    }
  }

  vec.sort_by_key(|&(_, size)| size );

  println!("{} directories are bigger than {} bytes.",
           vec.len().with_commas(), big_size.with_commas());

  let mut csv_writer = csv::Writer::from_file(csvfile).unwrap();

  csv_writer.encode(("Directory", "Size")).unwrap();

  for &(path_str, path_size) in vec.iter() {
    csv_writer.encode((path_str, path_size)).unwrap();
  }

  println!("Wrote {}.", csvfile.to_str().unwrap());
}
