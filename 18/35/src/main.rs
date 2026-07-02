impl Node {
  fn new() -> Self {
    Node {
      is_word: false,
      children: HashMap::new(),
    }
  }
}
