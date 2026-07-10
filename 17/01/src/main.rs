use std::io::Write;
use std::path::Path;
use std::{fs::*, io};

fn basic_file_handling() -> io::Result<()> {
  let path_loc = r"my_text.txt";
  let path = Path::new(path_loc);
  let mut file = File::create(path)?;

  file.write(b"let's put this in the file")?;
  file.write("let's put this in the file".as_bytes())?;

  Ok(())
}
fn main() -> io::Result<()> {
  basic_file_handling()
}
