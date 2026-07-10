use regex::Regex;

fn main() {
  let re = Regex::new(r"[a-z]ain").unwrap();
  let text = "main pain tain rain but not 0ain";
  for cap in re.captures_iter(text) {
    println!("match: {:?}", &cap[0]);
  }
}

