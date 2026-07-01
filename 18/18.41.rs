use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
struct Node {
    prod_id: i32,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(elem: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            prod_id: elem,
            prev: None,
            next: None,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;
#[derive(Default, Debug)]
struct DoublyLinkList {
    head: Link,
    tail: Link,
}

impl DoublyLinkList {
    fn new() -> DoublyLinkList {
        DoublyLinkList {
            head: None,
            tail: None,
        }
    }
    pub fn push_back(&mut self, elem: i32) -> Link {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.tail.clone()
    }

    pub fn remove_front(&mut self) -> Option<Link> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail.take();
                    None
                }
            }
        })
    }
}