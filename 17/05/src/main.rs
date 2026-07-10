use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{fs::*, io};

fn basic_file_handling() -> std::io::Result<()> {
  let path_loc = r"D:\my_text.txt";
  let path = Path::new(path_loc);
  let file = File::open(path)?;
  let file_buffer = BufReader::new(file);
  for lines in file_buffer.lines() {
    println!("{:?}", lines?);
  }
  Ok(())
}

fn main() -> io::Result<()> {
  basic_file_handling()
}
