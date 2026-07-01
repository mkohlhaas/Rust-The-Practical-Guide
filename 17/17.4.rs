...
fn basic_file_handling() -> std::io::Result<()> {
    let path_loc = r"D:\my_text.txt";
    let path = Path::new(path_loc);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("The file contains {:?}", contents);
    Ok(())
}