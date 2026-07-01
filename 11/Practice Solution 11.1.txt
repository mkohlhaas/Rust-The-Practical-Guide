#[derive(Debug)]
struct Linklist {
    head: pointer,
}
#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}
type pointer = Option<Box<Node>>;

impl Linklist {
    fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(H) => Some(H.element),
            None => None,
        }
    }
}
fn main() {} 