impl Node {
  fn new(element: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node {
      element: element,
      next: None,
      previous: None,
    }))
  }
}
