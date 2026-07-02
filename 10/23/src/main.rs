#[derive(Debug)]
enum List {
  Cons(i32, Option<Box<List>>),
}
