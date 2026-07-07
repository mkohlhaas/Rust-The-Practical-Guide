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

#[derive(Debug)]
struct Linkedlist {
  head: Node,
}

impl Linkedlist {
  fn new(head: Node) -> Self {
    Self { head }
  }
}

fn main() {
  // empty list still not possible
  // let list = Linkedlist::new(None); // ⚠️ Error

  let list1 = Linkedlist {
    head: Node {
      element: 1,
      next: Some(Box::new(Node {
        element: 2,
        next: Some(Box::new(Node {
          element: 3,
          next: None,
        })),
      })),
    },
  };

  let list2 = Linkedlist::new(Node::new(
    1,
    Some(Box::new(Node::new(2, Some(Box::new(Node::new(3, None)))))),
  ));

  println!("{:?}", list1);
  println!("{:?}", list2);
}

