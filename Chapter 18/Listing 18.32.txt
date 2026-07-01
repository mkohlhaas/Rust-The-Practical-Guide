impl<T: std::fmt::Debug> Linklist<T> {
    ...
    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut previous = None;
        let mut current_node = self.head.take();
        while current_node.is_some() {
            let next = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = previous.take();
            previous = current_node.take();
            current_node = next;
        }

        self.head = previous.take();
    }
}