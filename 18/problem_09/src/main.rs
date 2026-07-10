// NOTE: Code is wrong. Not in a mood of debugging this crap!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
  prod_id: i32,
  prev: Option<Link>,
  next: Option<Link>,
}

impl Node {
  fn new(elem: i32) -> Link {
    Rc::new(RefCell::new(Node {
      prod_id: elem,
      prev: None,
      next: None,
    }))
  }
}

#[derive(Default, Debug)]
struct DoublyLinkList {
  head: Option<Link>,
  tail: Option<Link>,
}

impl DoublyLinkList {
  fn new() -> DoublyLinkList {
    DoublyLinkList {
      head: None,
      tail: None,
    }
  }

  pub fn push_back(&mut self, elem: i32) -> Option<Link> {
    let new_tail = Node::new(elem);

    match self.tail.take() {
      Some(old_tail) => {
        old_tail.borrow_mut().next = Some(new_tail.clone());
        new_tail.borrow_mut().prev = Some(old_tail);
        self.tail = Some(new_tail);
      }
      None => {
        self.head = Some(new_tail.clone());
        self.tail = Some(new_tail);
      }
    }

    self.tail.clone()
  }

  /// Returns new head (or None).
  pub fn remove_front(&mut self) -> Option<Option<Link>> {
    self
      .head
      .take()
      .map(|old_head| match old_head.borrow_mut().next.take() {
        Some(new_head) => {
          new_head.borrow_mut().prev.take(); // set .prev of new head to None
          self.head = Some(new_head);
          self.head.clone()
        }
        None => None,
      })
  }

  fn move_to_tail(&mut self, node: &Link) {
    let prev = node.borrow().prev.as_ref().map(Rc::clone);
    let next = node.borrow().next.as_ref().map(Rc::clone);

    match (prev, next) {
      (None, None) => {}
      (Some(_), None) => {}
      (None, Some(next)) => {
        node.borrow_mut().next = None;
        next.borrow_mut().prev = None;
        self.head = Some(next.clone());

        let prev_tail = self.tail.as_ref().unwrap();
        prev_tail.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(prev_tail.clone());
        self.tail = Some(node.clone());
      }
      (Some(prev), Some(next)) => {
        node.borrow_mut().next = None;

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());

        let prev_tail = self.tail.as_ref().unwrap();
        prev_tail.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(prev_tail.clone());
        self.tail = Some(node.clone());
      }
    }
  }
}

#[derive(Debug)]
struct MRPProduct {
  map: HashMap<i32, Link>,
  prod_list: DoublyLinkList,
  size: i32,
  capacity: i32,
}

impl MRPProduct {
  fn new(capacity: i32) -> Self {
    Self {
      map: HashMap::new(),
      prod_list: DoublyLinkList::new(),
      size: 0,
      capacity,
    }
  }

  fn purchase(&mut self, prod_id: i32) {
    if let Some(node) = self.map.get(&prod_id) {
      self.prod_list.move_to_tail(node);
    } else {
      if self.size >= self.capacity {
        let new_head = self.prod_list.remove_front().unwrap();
        self.map.remove(&new_head.unwrap().borrow().prod_id);
      }
      let node = self.prod_list.push_back(prod_id).unwrap();
      self.map.insert(prod_id, node);
      self.size += 1;
    }
  }

  // TODO: Again! This should be a Display trait!
  fn print(&self) {
    let mut traversal = self.prod_list.head.clone();

    while traversal.is_some() {
      let temp = traversal.clone().unwrap();
      print!("{} ", temp.borrow().prod_id);
      traversal = temp.borrow().next.clone();
    }

    println!();
  }
}

fn main() {
  let mut prod_list = MRPProduct::new(3);

  print!("Purchasing 1: ");
  prod_list.purchase(1);
  prod_list.print();
  // println!("{:#?}", prod_list);
  println!("{:?}", prod_list.map.keys());

  print!("Purchasing 2: ");
  prod_list.purchase(2);
  prod_list.print();
  println!("{:?}", prod_list.map.keys());
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely

  print!("Purchasing 3: ");
  prod_list.purchase(3);
  prod_list.print();
  println!("{:?}", prod_list.map.keys());
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely

  print!("Purchasing 4: ");
  prod_list.purchase(4);
  prod_list.print();
  println!("{:?}", prod_list.map.keys()); // ⚠️ Wrong!
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely

  print!("Purchasing 3: ");
  prod_list.purchase(3);
  prod_list.print();
  println!("{:?}", prod_list.map.keys()); // ⚠️ Wrong!
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely

  print!("Purchasing 5: ");
  prod_list.purchase(5);
  prod_list.print();
  println!("{:?}", prod_list.map.keys()); // ⚠️ Wrong!
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely

  print!("Purchasing 2: ");
  prod_list.purchase(2);
  prod_list.print();
  println!("{:?}", prod_list.map.keys()); // ⚠️ Wrong!
  // println!("{:#?}", prod_list); // ⚠️ Error: will cycle indefinitely
}
