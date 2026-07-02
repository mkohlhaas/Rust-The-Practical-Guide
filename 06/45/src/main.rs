...
mod product {
    pub use category::Category;
    /// Struct for storing product relatedion information
    pub struct Product {
        id: u64,
        pub name: String, // make it pub so that the code in doc comments work
        price: f64,
        category: Category,
    }
    ...

    impl Product {
        /// # Example
        /// ```
        /// use my_package::Category;
        /// use my_package::Product;
        /// let some_product = Product::new(1, String::from("Laptop"), 799.9, Category::Electronics);
        /// assert_eq!(some_product.name, String::from("Laptop"));
        ///
        /// ```
    ...
}