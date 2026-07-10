use std::path::Path;

fn main() {
  let path = Path::new(r"my_file.txt");
  println!("Name of the file is {:?}", path.file_stem().unwrap());
  println!("Extension of the file is {:?}", path.extension().unwrap());
}

