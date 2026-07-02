impl Doubly_Linklist {
    ...
    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            ...
        }
    }
}