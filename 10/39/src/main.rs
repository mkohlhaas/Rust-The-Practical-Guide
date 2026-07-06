use std::ops::Deref;

// Deref coercion also works recursively.

struct InnerBox<T>(T);

impl<T> Deref for InnerBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

struct OuterBox<T>(T);

impl<T> Deref for OuterBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn print_value(value: &str) {
  println!("Value: {}", value);
}

fn main() {
  let nested: OuterBox<InnerBox<String>> = OuterBox(InnerBox(String::from("Nested Rust")));

  // deref coercion: &OuterBox -> &InnerBox -> &String -> &str
  print_value(&nested);
}
