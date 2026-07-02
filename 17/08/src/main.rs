use std::path::{Path, PathBuf};
fn main() {
  // approach 1
  let mut path = PathBuf::from(r"D:\");
  path.push(r"Rust");
  path.push(r"Examples");
  path.push(r"my_file");
  path.set_extension("txt");
  println!("The path is {:?}", path);

  // approach 2
  let path = [r"D:\", r"Rust", r"Examples", r"my_file.txt"]
    .iter()
    .collect::<PathBuf>();
  println!("The path is {:?}", path);
}
