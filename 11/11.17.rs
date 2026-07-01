impl Doubly_Linklist {
    ...
    fn add(&mut self, element: i32) {
        let new_head = Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            previous: None,
        }));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}