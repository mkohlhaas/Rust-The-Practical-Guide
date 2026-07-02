pub use crate::product::Category;
pub use customer::Customer;
pub use product::Product;

mod product {
    pub use category::Category;
    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        category: category::Category,
    }
...
}
...