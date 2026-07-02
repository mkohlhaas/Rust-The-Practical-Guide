mod product {
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: category::Category, // Error
    }
  ...
}
...