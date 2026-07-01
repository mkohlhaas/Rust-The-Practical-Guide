impl DisplayLobby {
    ...
    fn next_name(&mut self) -> String {
        let node = self.stack.pop().unwrap();
        let name = &node.borrow().val;
        let mut next_node = node.borrow().right.clone();
        Self::push_all_left(next_node, &mut self.stack);
        name.to_string()
    }
} 