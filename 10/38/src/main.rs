use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(value: T) -> MyBox<T> {
    MyBox(value)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// NOTE: takes a &str
fn greet(name: &str) {
  println!("Hello, {}!", name);
}

fn main() {
  let my_box = MyBox::new(String::from("Rust"));

  // NOTE: we give `greet` y MyBox
  greet(&my_box); // deref coercion: &MyBox -> &String -> &str: "Hello, Rust!"
}
