impl MRPProduct {
    ...
    fn print(&self) {
        let mut traversal = self.product_list.head.clone();
        while !traversal.is_none() {
            let temp = traversal.clone().unwrap();
            print!("{} ", temp.borrow().prod_id);
            traversal = temp.borrow().next.clone();
        }
        println!("");
    }
}