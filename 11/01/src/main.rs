#[derive(Debug)]
struct Node {
  element: i32,
  next: Option<Box<Node>>,
}
