#![allow(dead_code, unused_variables)]

use std::rc::Rc;

#[derive(Debug)]
enum List {
  Cons(i32, Option<Rc<List>>),
}

fn main() {
  let a: Rc<List> = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
  println!("Reference count after a: {}", Rc::strong_count(&a)); // 1
  // println!("{:?}", a);

  {
    let b: List = List::Cons(3, Some(Rc::clone(&a)));
    println!("Reference count after b: {}", Rc::strong_count(&a)); // 2
    // println!("{:?}", b);

    let c: List = List::Cons(4, Some(Rc::clone(&a)));
    println!("Reference count after c: {}", Rc::strong_count(&a)); // 3
    // println!("{:?}", c);
  }

  println!("Reference count after scope: {}", Rc::strong_count(&a)); // 1
  // println!("{:?}", a);
}
