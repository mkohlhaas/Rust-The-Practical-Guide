fn main() {
  let book = Book {
    title: "Digital Image Processing".to_string(),
    author: "Gonzales".to_string(),
    genre: "Science Book".to_string(),
  };
  let mut book_iterator = book.into_iter();
  println!("{:?}", book_iterator.next());
  println!("{:?}", book_iterator.next());
  println!("{:?}", book_iterator.next());
  println!("{:?}", book_iterator.next());
}
