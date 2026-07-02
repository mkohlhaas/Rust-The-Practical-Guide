impl Linkedlist {
    ...
    fn add(&mut self, element: i32) {
        match self.head { // Error
            None => {
                let new_node = Some(Box::new(Node {
                    element: element,
                    next: None,
                }));
                self.head = new_node
            }
            Some(previous_head) => {
                let new_node = Some(Box::new(Node {
                    element: element,
                    next: Some(previous_head),
                }));
                self.head = new_node;
            }
        }
    }
}