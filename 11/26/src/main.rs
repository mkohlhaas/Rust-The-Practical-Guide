fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            ...
        } else {
           self.head
               .take()
               .map(|old_head| {});
        }
    }
}