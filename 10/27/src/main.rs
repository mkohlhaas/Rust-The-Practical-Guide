#![allow(dead_code, unused_variables)]

use std::rc::Rc;

// Rc is Clone, but not Copy
enum List {
  Cons(i32, Option<Rc<List>>),
}

fn main() {
  // {
  //   let a: List = List::Cons(1, Some(Rc::new(List::Cons(2, None))));
  //   let b = List::Cons(3, Some(Rc::clone(&a))); // ⚠️ Error
  //   let c = List::Cons(4, Some(Rc::clone(&a))); // ⚠️ Error
  // }

  {
    // the compiler now treats a, b, and c all as owners of the list
    let a: Rc<List> = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    let b = List::Cons(3, Some(Rc::clone(&a)));
    let c = List::Cons(4, Some(Rc::clone(&a)));
  }
}
