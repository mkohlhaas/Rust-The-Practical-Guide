use regex::Regex;

fn main() {
  let re = Regex::new(r"\d\d\d\d\d\d").unwrap();
  let text = "My phone number is 816030 and the second phone number is 816694";

  for cap in re.captures_iter(text) {
    println!("match: {:?}", &cap[0]);
  }
}
