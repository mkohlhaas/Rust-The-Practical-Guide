fn main() {
    let product = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
    let suggestions = product_suggestions(product, 50);
    println!("{:?}", suggestions);
}