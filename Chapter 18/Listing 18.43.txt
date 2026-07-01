impl MRPProduct {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            product_list: DoublyLinkList::new(),
            size: 0,
            capacity: capacity,
        }
    }
}