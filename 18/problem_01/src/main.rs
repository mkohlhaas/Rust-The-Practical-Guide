use std::collections::HashMap;

fn word_grouping(words_list: Vec<String>) -> Vec<Vec<String>> {
  let mut word_hash = HashMap::new();

  for word in words_list {
    let mut char_freq = vec![0; 26];

    for c in word.to_lowercase().chars() {
      char_freq[(c as u32 - 'a' as u32) as usize] += 1;
    }

    let key: String = char_freq
      .into_iter()
      .map(|n| n.to_string())
      .collect::<String>();

    println!("key: {key}");

    word_hash.entry(key).or_insert(Vec::new()).push(word);
  }

  println!("{:#?}", word_hash);

  word_hash.into_iter().map(|(_key, words)| words).collect()
}

fn main() {
  {
    let foo = vec!["abc", "def", "ghi"];
    let foo = foo.into_iter().collect::<String>();
    assert_eq!(foo, "abcdefghi");
  }

  let words = vec![
    "The".to_string(),
    "teh".to_string(),
    "het".to_string(),
    "stupid".to_string(),
    "studpi".to_string(),
    "apple".to_string(),
    "appel".to_string(),
  ];

  let input_word = String::from("teh");
  let grouping = word_grouping(words);

  println!("{:?}", grouping);

  for words in grouping {
    if words.contains(&input_word) {
      println!("The anagrams of the word '{input_word}' is {:?}.", words);
    }
  }
}
