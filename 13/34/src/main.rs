#![allow(dead_code)]

use std::{marker::PhantomData, rc::Rc};

// Abc1 is Send and Sync
// size: 0
#[derive(Debug)]
struct Abc1;

// Abc2 is !Send and !Sync
// size: 8
#[derive(Debug)]
struct Abc2 {
  ensuring_no_send_sync: Rc<()>,
}

// Abc3 is !Send and !Sync
// size: 0
#[derive(Debug)]
struct Abc3 {
  ensuring_no_send_sync: PhantomData<Rc<()>>,
}

fn main() {
  let abc1 = Abc1;

  let abc2 = Abc2 {
    ensuring_no_send_sync: Rc::new(()),
  };

  let abc3 = Abc3 {
    ensuring_no_send_sync: PhantomData,
  };

  println!("{:?}", abc1);
  println!("{:?}", abc2);
  println!("{:?}", abc3);

  println!();

  println!("{}", size_of::<Abc1>()); // 0
  println!("{}", size_of::<Abc2>()); // 8
  println!("{}", size_of::<Abc3>()); // 0
}
