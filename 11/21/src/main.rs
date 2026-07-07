impl Doubly_Linklist {
    ...
    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so we can not remove");
            None
        } else {
	     ...
        }
    }
}
