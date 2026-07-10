use std::path::Path;

fn main() {
  let path = Path::new(r"my_file.txt");
  println!("Folder containing the file: {:?}", path.parent().unwrap());
}
