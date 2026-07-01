mod product {
    ...

    impl Product {
        ...
        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }
}