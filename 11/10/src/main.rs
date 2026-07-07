#[allow(dead_code)]
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
}

fn main() {
  let mut list = Linkedlist::new();

  list.add(5);
  list.add(4);
  list.add(3);
  list.add(2);
  list.add(1);

  println!("List: {:#?}", list);
}
