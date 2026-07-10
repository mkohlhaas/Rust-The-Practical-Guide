use regex::Regex;

fn main() {
  let re = Regex::new(r"^aba").unwrap();
  let text = "aba aba bc";

  for cap in re.captures_iter(text) {
    println!("match: {:?}", &cap[0]);
  }
}
