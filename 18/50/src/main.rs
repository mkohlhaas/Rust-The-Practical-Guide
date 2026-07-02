use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
  val: String,
  left: Link,
  right: Link,
}
type Link = Option<Rc<RefCell<Node>>>;
impl Node {
  fn new(val: String) -> Self {
    Node {
      val,
      left: None,
      right: None,
    }
  }
  fn insert(&mut self, val: String) {
    if val > self.val {
      match &self.right {
        None => self.right = Some(Rc::new(RefCell::new(Self::new(val)))),
        Some(node) => node.borrow_mut().insert(val.to_string()),
      }
    } else {
      match &self.left {
        None => self.left = Some(Rc::new(RefCell::new(Self::new(val)))),
        Some(node) => node.borrow_mut().insert(val.to_string()),
      }
    }
  }
}
