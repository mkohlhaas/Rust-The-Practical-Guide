// Trie Data Structure

use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Node {
  is_word: bool,
  children: HashMap<char, Node>,
}

impl Node {
  fn new() -> Self {
    Node {
      is_word: false,
      children: HashMap::new(),
    }
  }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct WordDictionary {
  root: Node,
}

impl WordDictionary {
  fn new() -> Self {
    Self::default()
  }

  fn insert(&mut self, word: &str) {
    let mut current_node = &mut self.root;

    for char in word.chars() {
      current_node = current_node.children.entry(char).or_insert(Node::new());
    }

    current_node.is_word = true;
  }

  fn search(&self, word: &str) -> bool {
    let mut current_node = &self.root;

    for char in word.chars() {
      let next_node = current_node.children.get(&char);

      if next_node.is_some() {
        current_node = next_node.unwrap()
      } else {
        return false;
      }
    }

    current_node.is_word
  }
}

fn main() {
  let mut dict = WordDictionary::new();
  let words = vec![
    "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
  ];

  for word in words {
    dict.insert(&word);
  }

  // println!("{:#?}", dict);

  assert!(dict.search("there"));
  assert!(dict.search("answer"));

  assert!(!dict.search("car"));
  assert!(!dict.search("auto"));
  assert!(!dict.search("them"));
  assert!(!dict.search("anyway"));
}
