use std::path::Path;
fn main() {
  let path = Path::new(r"D:\my_text.txt");
  let data = path.metadata().unwrap();
  println!("type {:?}", data.file_type());
  println!("length {:?}", data.len());
  println!("Permissions {:?}", data.permissions());
  println!("Modified {:?}", data.modified());
  println!("Created {:?}", data.created());
}
