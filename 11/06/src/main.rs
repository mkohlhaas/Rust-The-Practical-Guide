#![allow(dead_code)]

type Pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
  element: i32,
  next: Pointer,
}

impl Node {
  fn new(element: i32, next: Pointer) -> Self {
    Self { element, next }
  }
}

#[derive(Debug)]
struct Linkedlist {
  head: Option<Node>,
}

impl Linkedlist {
  fn new(head: Option<Node>) -> Self {
    Self { head }
  }

  fn empty() -> Self {
    Self::new(None)
  }
}

fn main() {
  // empty list now possible
  let list1 = Linkedlist::new(None);
  let list2 = Linkedlist::empty();

  let list3 = Linkedlist {
    head: Some(Node {
      element: 1,
      next: Some(Box::new(Node {
        element: 2,
        next: Some(Box::new(Node {
          element: 3,
          next: None,
        })),
      })),
    }),
  };

  let list4 = Linkedlist::new(Some(Node::new(
    1,
    Some(Box::new(Node::new(2, Some(Box::new(Node::new(3, None)))))),
  )));

  println!("{:?}", list1);
  println!("{:?}", list2);
  println!("{:?}", list3);
  println!("{:?}", list4);
}
