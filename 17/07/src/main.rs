use std::path::{Path, PathBuf};
fn main() {
    ...
    println!("Name of the file is {:?}", path.file_stem().unwrap());
    println!("Extension of the file is {:?}", path.extension().unwrap());
}