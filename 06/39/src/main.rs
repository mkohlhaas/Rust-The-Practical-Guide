pub use crate::product::Category;
pub use customer::Customer;
pub use product::Product;
mod product {
    ...
    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Self {
            Self {
                id,
                name,
                price,
                category,
            }
        }
        ...
    }
}
...