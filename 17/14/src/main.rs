use regex::Regex;
fn main() {
  let re = Regex::new(r"[prt]ain").unwrap();
  let text = "rrrain spain none";
  for cap in re.captures_iter(text) {
    println!("match: {:?}", &cap[0]);
  }
}
