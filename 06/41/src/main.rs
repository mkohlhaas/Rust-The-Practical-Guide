use my_package_book::{Category, Customer, Product};
fn main() {
  let product = Product::new(1, String::from("Laptop"), 799.99, Category::Electronics);

  let customer = Customer::new(1, String::from("Alice"), String::from("alice@example.com"));
}
