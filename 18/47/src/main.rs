impl MRPProduct {
    ...
    fn purchase(&mut self, prod_id: i32) {
        if let Some(node) = self.map.get(&prod_id) {
            self.product_list.move_to_tail(node);
        } else {
            if self.size >= self.capacity {
                let prev_head = self.product_list.remove_front().unwrap();
                self.map.remove(&prev_head.unwrap().borrow().prod_id);
            }
            let node = self.product_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id, node);
            self.size += 1;
        }
    }
}