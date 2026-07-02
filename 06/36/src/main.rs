use my_package::{Category, Customer, Product};
fn main() {
  let product = Product {
    id: 1,                           // Error
    name: String::from("Laptop"),    // Error
    price: 799.99,                   // Error
    category: Category::Electronics, // Error
  };
}
