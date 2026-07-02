use regex::Regex;
fn main() {
    let re = Regex::new(r"gr[ae]y").unwrap();
    let text = "gray grey graye";
    for cap in re.captures_iter(text) {
        println!("match: {:?}", &cap[0]);
    }
}