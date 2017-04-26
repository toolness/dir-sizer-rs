extern crate csv;

use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

use util::WithCommas;
use reporter::Reporter;

pub struct DirMapper {
  sizes: HashMap<String, u64>,
  path: PathBuf
}

impl DirMapper {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self {
      sizes: HashMap::new(),
      path: path.as_ref().to_path_buf()
    }
  }

  pub fn create_csvfile<P: AsRef<Path>>(
    &mut self,
    csvfile: P,
    mut reporter: &mut Reporter
  ) {
    let csvfile = csvfile.as_ref();

    self.load_or_create_csvfile(csvfile);

    let file = fs::OpenOptions::new().append(true).open(csvfile).unwrap();
    let mut csv_writer = csv::Writer::from_writer(file);
    let root_path = self.path.clone();
    let size = self.get_dir_size(
      &mut csv_writer,
      root_path.as_path(),
      &mut reporter
    );

    println!("\nTotal size of {} is {} bytes.",
             self.path.to_string_lossy(), size.with_commas());
  }

  pub fn create_big_csvfile<P: AsRef<Path>>(
    &mut self,
    csvfile: P,
    big_size: u64
  ) {
    let csvfile = csvfile.as_ref();

    let mut vec = Vec::new();
    let root_path_str = self.path.to_str().unwrap();

    for (path_str, &path_size) in self.sizes.iter() {
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

    println!("Wrote {}.", csvfile.to_string_lossy());
  }

  fn load_or_create_csvfile(&mut self, csvfile: &Path) {
    if csvfile.exists() {
      println!("Loading {}...", csvfile.to_string_lossy());
      let mut reader = csv::Reader::from_file(csvfile).unwrap();
      for record in reader.decode() {
        let (path_str, size): (String, u64) = record.unwrap();
        self.sizes.insert(path_str, size);
      }
      println!("Loaded {} record(s).", self.sizes.len().with_commas());
    } else {
      let mut file = fs::File::create(csvfile).unwrap();
      file.write_all(b"Directory,Size\n").unwrap();
    }
  }

  fn get_dir_size(&mut self,
                  writer: &mut csv::Writer<fs::File>,
                  path: &Path,
                  reporter: &mut Reporter) -> u64 {
    let path_str = path.to_str().unwrap();

    match self.sizes.get(path_str) {
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
            total += self.get_dir_size(writer, &subpath, reporter);
          } else {
            total += reporter.count_bytes(metadata.len());
          }
        }
      },
      Err(e) => {
        reporter.error_accessing(path_str, e);
      },
    }

    self.sizes.insert(String::from(path_str), total);
    writer.encode((path_str, total)).unwrap();

    total
  }
}
