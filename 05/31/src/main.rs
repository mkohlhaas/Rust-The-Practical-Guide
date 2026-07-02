let target_word = "Hello";
let mut found = false;
for (word, _) in &word_counts {
    if word.to_lowercase() == target_word.to_lowercase() {
        found = true;
        break;
     }
}