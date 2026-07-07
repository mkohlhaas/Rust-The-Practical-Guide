#![allow(dead_code)]

#[derive(Debug)]
struct Node {
  element: i32,
  next: Pointer,
}

type Pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Linkedlist {
  head: Pointer,
}

impl Linkedlist {
  fn new() -> Self {
    Self { head: None }
  }

  fn add(&mut self, element: i32) {
    let previous_head = self.head.take();
    let new_head = Some(Box::new(Node {
      element: element,
      next: previous_head,
    }));
    self.head = new_head;
  }

  fn remove(&mut self) -> Option<i32> {
    match self.head.take() {
      Some(previous_head) => {
        self.head = previous_head.next;
        Some(previous_head.element)
      }
      None => None,
    }
  }

  // Variable                         | Type
  // ---------------------------------+--------------------
  // list_traversal                   | &Option<Box<Node>>
  // list_traversal.unwrap()          | Box<Node>
  // list_traversal.as_ref()          | Option<&Box<Node>>
  // list_traversal.as_ref().unwrap() | &Box<Node>

  // TODO: implement Display trait
  // https://medium.com/@shivamojha2419/100-days-of-rust-day-16-singly-linked-list-305461f282e9
  fn printing(&self) {
    let mut list_traversal = &self.head;
    while list_traversal.is_some() {
      println!("{:?}", list_traversal.as_ref().unwrap().element);
      list_traversal = &list_traversal.as_ref().unwrap().next;
    }
  }
}

fn main() {
  let mut list = Linkedlist::new();

  list.add(5);
  list.add(4);
  list.add(3);
  list.add(2);
  list.add(1);

  println!("List: {:#?}", list);
  list.printing();

  println!("{:?}", list.remove()); // Some(1)
  println!("{:?}", list.remove()); // Some(2)
  println!("{:?}", list.remove()); // Some(3)
  println!("{:?}", list.remove()); // Some(4)
  println!("{:?}", list.remove()); // Some(5)
  println!("{:?}", list.remove()); // None
  println!("{:?}", list.remove()); // None
  list.printing();
}
