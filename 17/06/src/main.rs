use std::path::{Path, PathBuf};
fn main() {
  let path = Path::new(r"D:\Rust\Examples\my_file.txt");
  println!("Folder containing the file: {:?}", path.parent().unwrap());
}
