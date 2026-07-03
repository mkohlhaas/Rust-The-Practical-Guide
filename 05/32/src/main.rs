use std::collections::HashMap;

fn main() {
  let mut word_counts: HashMap<&str, u8> = HashMap::new();
  word_counts.insert("Hello", 5);
  word_counts.insert("world", 2);
  word_counts.insert("Rust", 15);
  word_counts.insert("Programming", 5);

  let target_word = "Hello";
  let mut found = false;

  for (word, _) in &word_counts {
    if word.to_lowercase() == target_word.to_lowercase() {
      found = true;
      break;
    }
  }

  println!("Found: {found}");
}

