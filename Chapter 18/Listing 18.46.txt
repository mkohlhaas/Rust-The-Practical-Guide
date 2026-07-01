impl DoublyLinkList {
    ...
    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (prev, next) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(next)) => {
                node.borrow_mut().next = None;
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;

                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
        }
    }
}