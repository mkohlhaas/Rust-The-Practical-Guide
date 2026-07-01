pub use crate::product::Category;
pub use customer::Customer;
pub use product::Product;

mod product {
    pub use category::Category;
    pub struct Product {
        pub id: u64,
        pub name: String,
        pub price: f64,
        pub category: category::Category,
    }
...
}
...