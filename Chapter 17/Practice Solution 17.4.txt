use std::path::{Path, PathBuf};
fn main() {
    let components = vec!["C:", "Users", "Public", "example.txt"];
    let mut path = PathBuf::new();
    // Task 1: 
    for component in components {
        path.push(component);
    }
    println!("Constructed Path: {:?}", path);

    // Task 2: 
    if path.is_file() {
        println!("The path exists as a file.");
        let data = path.metadata().unwrap();
        println!("type {:?}", data.file_type());
        println!("length {:?}", data.len());
        println!("Permissions {:?}", data.permissions());
        println!("Modified {:?}", data.modified());
        println!("Created {:?}", data.created());
    } else {
        println!("The path does not exist as a file.");
    }
} 