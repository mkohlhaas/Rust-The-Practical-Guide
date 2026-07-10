use std::{fs::Metadata, path::Path};

fn main() {
  let path = Path::new(r"my_text.txt");
  let data: Metadata = path.metadata().unwrap();
  println!("type {:?}", data.file_type());
  println!("length {:?}", data.len());
  println!("Permissions {:?}", data.permissions());
  println!("Modified {:?}", data.modified());
  println!("Created {:?}", data.created());
}
