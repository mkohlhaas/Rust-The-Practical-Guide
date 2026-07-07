#![allow(dead_code)]

#[derive(Debug)]
struct Linkedlist {
  head: Pointer,
}

#[derive(Debug)]
struct Node {
  element: i32,
  next: Pointer,
}

type Pointer = Option<Box<Node>>;

fn main() {}
