mod product {
    struct Product {
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }
    
    mod category {
        enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }
    impl Product {
	...
}
...