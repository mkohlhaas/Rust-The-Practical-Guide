#[derive(Debug)]
struct Linkedlist {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}
type pointer = Option<Box<Node>>; 