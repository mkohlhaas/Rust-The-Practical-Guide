impl MaxStack {
    ...
    fn pop(&mut self) {
        self.main_stack.pop();
        self.max_stack.pop();
    }
}