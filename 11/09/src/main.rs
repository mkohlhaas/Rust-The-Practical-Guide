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
  head: Pointer,
}

impl Linkedlist {
  fn new(head: Pointer) -> Self {
    Self { head }
  }

  fn empty() -> Self {
    Self::new(None)
  }

  // fn add(&mut self, element: i32) {
  //   match self.head {
  //     None => {
  //       let new_node = Some(Box::new(Node {
  //         element: element,
  //         next: None,
  //       }));
  //       self.head = new_node
  //     }
  //     // ⚠️ Error: move happens here in Some(…)
  //     Some(previous_head) => {
  //       let new_node = Some(Box::new(Node {
  //         element: element,
  //         next: Some(previous_head),
  //       }));
  //       self.head = new_node;
  //     }
  //   }
  // }
}

fn main() {
  let list1 = Linkedlist::new(None);
  let list2 = Linkedlist::empty();

  let list3 = Linkedlist {
    head: Some(Box::new(Node {
      element: 1,
      next: Some(Box::new(Node {
        element: 2,
        next: Some(Box::new(Node {
          element: 3,
          next: None,
        })),
      })),
    })),
  };

  let list4 = Linkedlist::new(Some(Box::new(Node::new(
    1,
    Some(Box::new(Node::new(2, Some(Box::new(Node::new(3, None)))))),
  ))));

  println!("{:?}", list1);
  println!("{:?}", list2);
  println!("{:?}", list3);
  println!("{:?}", list4);

  println!("{:?}", &list4.head);
  println!("{:?}", &list4.head.as_ref().unwrap().element); // 1
  println!(
    "{:?}",
    &list4.head.as_ref().unwrap().next.as_ref().unwrap().element // 2
  );
  println!(
    "{:?}",
    &list4.head.unwrap().next.unwrap().next.unwrap().element // 3
  );
}
