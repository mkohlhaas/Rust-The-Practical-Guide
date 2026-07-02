use std::path::{Path, PathBuf};
fn main() {
  let path = Path::new(r"D:\Rust_learning");
  println!("Is the path a directory {:?}", path.is_dir());

  let path = Path::new(r"D:\my_text.txt");
  println!("Does the file exists: {:?}", path.is_file());
}
