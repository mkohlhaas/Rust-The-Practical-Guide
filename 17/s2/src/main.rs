fn main() {
  let re = Regex::new(r"_?\w+\.rs").unwrap();
  let text = "file.rs _file.rs test.rs _test.rs";

  for cap in re.find_iter(text) {
    println!("{}", cap.as_str());
  }
}
