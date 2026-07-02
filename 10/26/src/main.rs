enum List {
  Cons(i32, Option<Box<List>>),
}
fn main() {
  let a = List::Cons(1, Some(Box::new(List::Cons(2, None))));
  let b = List::Cons(3, Some(Box::new(a)));
  let c = List::Cons(4, Some(Box::new(a))); // Error
}
