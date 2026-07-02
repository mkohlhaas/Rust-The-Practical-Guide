mod product {
    pub struct Product { // Changed to public
        id: u64,
        name: String,
        price: f64,
        category: Category,
    }
  ...
}

mod customer {
    pub struct Customer { // Changed to public
        id: u64,
        name: String,
        email: String,
    }
}
...