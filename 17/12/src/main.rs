use std::env;
use std::fs;
//use std::path::Path;
fn main() {
  let mut curr_path = env::current_dir().expect("can't access current directory");
  println!("{:?}", curr_path);

  println!("Create a new directory: {:?}", fs::create_dir(r"D:\rust1"));
  println!(
    "Create a new directory and sub directories: {:?}",
    fs::create_dir_all(r"D:\rust1\level1\level2")
  );

  println!(
    "Remove a specific directory: {:?}",
    fs::remove_dir(r"D:\rust1\level1\level2")
  );
  println!(
    "Remove a specific directory when it is not empty {:?}",
    fs::remove_dir(r"D:\rust1")
  );
  println!(
    "Remove everything from a directory {:?}",
    fs::remove_dir_all(r"D:\rust1")
  );
  println!("Remving a file {:?}", fs::remove_file(r"D:\my_text.txt"));
  println!(
    "Renaming a file {:?}",
    fs::rename(r"D\prev.txt", r"D:\new.txt")
  );
  println!(
    "Copying contents from one file to another: {:?}",
    fs::copy(r"D:\new1.txt", r"D:\new2.txt")
  );
}
