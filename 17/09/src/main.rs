use std::path::Path;

fn main() {
  let path = Path::new(r".");
  println!("Is the path a directory {:?}", path.is_dir());

  let path = Path::new(r"my_text.txt");
  println!("Does the file exists: {:?}", path.is_file());
}
