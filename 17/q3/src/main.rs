fn main() {
  let re = Regex::new(r"// Add your regex here").unwrap();
  let text = "eager beaver sees three elephants";
  for cap in re.find_iter(text) {
    println!("{}", cap.as_str());
  }
}
