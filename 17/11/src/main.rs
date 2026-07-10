use std::path::Path;

fn main() {
  let path = Path::new(r".");
  for files in path.read_dir().expect("read_dir call failed") {
    println!("{:?}", files.unwrap().path());
  }
}
