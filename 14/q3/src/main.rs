use std::thread;
fn main() {
  let mut v = vec!["Nouman".to_string()];
  let handle = thread::spawn(|| {
    v.push("Azam".to_string());
  });
}
