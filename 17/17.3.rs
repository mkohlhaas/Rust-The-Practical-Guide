...
fn basic_file_handling() -> std::io::Result<()> {
    let path_loc = r"D:\my_text.txt";
    let path = Path::new(path_loc);
       let mut file = OpenOptions::new().append(true).open(path)?;

    // Storing string in a variable
    let str1 = "some text";
    file.write(str1.as_bytes())?;

    // Storing data in a vector
    let some_vec = vec![1,2,3,4,5,6];
    let str_from_vec = some_vec
    .into_iter()
    .map(|a| format!("{} ", a.to_string()))
    .collect::<String>();
    file.write(str_from_vec.as_bytes())?;

    // Storing data contained in multiple variables
    let (name, age) = ("Joseph", 40);
    let formatted_str = format!("I am {} and my name is {}", name, age);
    file.write(formatted_str.as_bytes())?;
    Ok(())
}