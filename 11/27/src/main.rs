impl Doubly_Linklist {
    ...
    fn remove(&mut self) -> Option<i32> {
        ...
        } else {
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() { });
        }
    }
}