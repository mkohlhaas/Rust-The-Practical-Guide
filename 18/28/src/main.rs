#[derive(Debug)]
struct Linklist<T: std::fmt::Debug> {
  head: pointer<T>,
}
#[derive(Debug)]
struct Node<T> {
  element: T,
  next: pointer<T>,
}
type pointer<T> = Option<Box<Node<T>>>;
impl<T: std::fmt::Debug> Linklist<T> {
  fn create_empty_list() -> Linklist<T> {
    Linklist { head: None }
  }
  fn add(&mut self, element: T) {
    let previous_head = self.head.take();
    let new_head = Box::new(Node {
      element: element,
      next: previous_head,
    });
    self.head = Some(new_head);
  }
  fn remove(&mut self) -> Option<T> {
    let previous_head = self.head.take();
    match previous_head {
      Some(old_head) => {
        self.head = old_head.next;
        Some(old_head.element)
      }
      None => None,
    }
  }
  fn peek(&self) -> Option<&T> {
    match &self.head {
      Some(H) => Some(&H.element),
      None => None,
    }
  }
  fn printing(&self) {
    let mut list_traversal = &self.head;
    println!();

    while true {
      match list_traversal {
        Some(Node) => {
          print!("{:?} ", Node.element);
          list_traversal = &list_traversal.as_ref().unwrap().next;
        }
        None => break,
      }
    }
  }
}
