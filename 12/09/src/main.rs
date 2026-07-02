impl Customer {
  fn new(name: String) -> Self {
    Customer {
      name: name,
      ..Default::default()
    }
  }
}
