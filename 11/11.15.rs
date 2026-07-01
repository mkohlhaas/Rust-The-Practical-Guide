use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct DoublyLinkedlist {
    head: pointer,
    tail: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
    previous: pointer,
}
type pointer = Option<Rc<RefCell<Node>>>;