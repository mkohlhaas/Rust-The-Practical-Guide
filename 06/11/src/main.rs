...
mod Order {
    ...
    impl Order {
     ...
        fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64; // Error
            total_before_discount - (total_before_discount * discount)
        }
    }
}