use std::ops::Deref;
struct InnerBox<T>(T);
struct OuterBox<T>(T);
impl<T> Deref for InnerBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Deref for OuterBox<T> {
    type Target = InnerBox<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn print_value(value: &str) {
    println!("Value: {}", value);
}
fn main() {
    let nested = OuterBox(InnerBox(String::from("Nested Rust")));
    print_value(&nested); // Multiple levels of deref coercion
}