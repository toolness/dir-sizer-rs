extern crate csv;

use std::path::Path;
use std::fs;
use std::collections::HashMap;

const ROOT_DIR: &'static str = "C:\\";

fn main() {
  let mut map = HashMap::new();
  let path = Path::new(ROOT_DIR);

  map.insert(path, 150);

  let outfile = Path::new("dirs.csv");

  let mut writer = csv::Writer::from_file(outfile).unwrap();

  for (key, val) in map.iter() {
    let result = writer.encode((key.to_str(), val));
    assert!(result.is_ok());
  }

  assert!(writer.flush().is_ok());

  let infile = Path::new("dirs.csv");

  let mut reader = csv::Reader::from_file(infile).unwrap()
    .has_headers(false);

  for record in reader.decode() {
    let (path_str, size): (String, u32) = record.unwrap();
    println!("Neat {} = {}", path_str, size);
  }

  let result = fs::read_dir(path);

  match result {
    Ok(v) => println!("AW YISS {:?}", v),
    Err(e) => println!("AW SNAP {:?}", e),
  }

  println!("SUP world. {path:?}", path=path);
}
