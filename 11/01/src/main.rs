#![allow(dead_code)]

#[derive(Debug)]
struct Node {
  element: i32,
  next: Option<Box<Node>>,
}

impl Node {
  fn new(element: i32, next: Option<Box<Node>>) -> Self {
    Self { element, next }
  }
}

fn main() {
  let list1 = Node::new(1, None);

  let list2 = Node {
    element: 1,
    next: Some(Box::new(Node {
      element: 2,
      next: Some(Box::new(Node {
        element: 3,
        next: None,
      })),
    })),
  };

  let list3 = Node::new(
    1,
    Some(Box::new(Node::new(
      2,
      Some(Box::new(Node::new(
        3, //
        Some(Box::new(Node::new(4, None))),
      ))),
    ))),
  );

  println!("{:?}", list1);
  println!("{:?}", list2);
  println!("{:#?}", list3);
}
