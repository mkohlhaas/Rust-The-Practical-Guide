#[derive(Debug, Default, PartialEq, Eq)]
struct BinarySearchTree {
  root: Node,
}
impl BinarySearchTree {
  fn new(val: String) -> Self {
    BinarySearchTree {
      root: Node::new(val.to_string()),
    }
  }
  fn insert(&mut self, val: String) {
    self.root.insert(val.to_string());
  }
}
