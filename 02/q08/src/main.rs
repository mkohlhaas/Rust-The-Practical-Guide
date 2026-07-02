// Defining a type alias for a tuple

fn main() {
  type Book = (String, String, u16); // Add your code here to this line
  let book1: Book = (
    String::from("Rust Programming Language"),
    String::from("RUST Community"),
    2010,
  );

  println!(
    "Book name: {}, Author: {}, Year {}",
    book1.0, book1.1, book1.2
  );
}
