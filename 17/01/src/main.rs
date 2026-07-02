use std::fs::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

fn basic_file_handling() -> std::io::Result<()> {
  let path_loc = r"D:\my_text.txt";
  let path = Path::new(path_loc);
  let mut file = File::create(path)?;

  file.write(b"let's put this in the file")?;
  file.write("let's put this in the file".as_bytes())?;

  Ok(())
}
fn main() {
  basic_file_handling();
}
