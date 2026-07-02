impl Node {
    ...
    fn insert(&mut self, value: i32) {
        if value > self.val {
            match self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            }
        } else {
            match self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            }
        }
    }
}