impl MaxStack {
    ...
    fn max_value(&self) -> i32 {
        *self.max_stack.last().unwrap()
    }
}