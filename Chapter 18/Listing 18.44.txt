impl MRPProduct {
    ...
    fn purchase(&mut self, prod_id: i32) {
        if let Some(node) = self.map.get(&prod_id) {
            self.item_list.move_to_tail(node);
        }
    }
}