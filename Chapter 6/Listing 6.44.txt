...
mod product {
    pub use category::Category;
    /// Struct for storing product relatedion information
    pub struct Product {
    ...
    }
    mod category {
        /// Enum for representing product categories.
        pub enum Category {
     ...
        }
    }
}
...