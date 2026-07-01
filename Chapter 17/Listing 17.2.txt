...
fn basic_file_handling() -> std::io::Result<()> {
    let path_loc = r"D:\my_text.txt";
    let path = Path::new(path_loc);
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write("\n www.includehelp.com\n".as_bytes())?;
    Ok(())
}
fn main() {
    basic_file_handling();
}