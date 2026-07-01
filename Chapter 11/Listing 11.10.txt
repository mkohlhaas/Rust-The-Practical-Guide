impl Linkedlist {
    ...
    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_node = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }
}