fn main() {
  type Book = (String, String, u32);
  let book1: Book = (
    String::from("Rust Programming Language"),
    String::from("RUST Community"),
    2020,
  );
  println!(
    "Book name: {}, Author: {}, Year {}",
    book1.0, book1.1, book1.2
  );
}
