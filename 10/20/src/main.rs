#[derive(Debug)]
enum List {
  Cons(i32, List), // ⚠️ Error
  Nil,
}

fn main() {
  let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
  println!("{:?}", list);
}
