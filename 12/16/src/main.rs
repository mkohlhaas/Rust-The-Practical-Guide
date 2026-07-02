impl Customer {
  fn new(name: String) -> CustomerBuilder {
    CustomerBuilder {
      name: name,
      username: None,
      membership: None,
      gender: None,
      country: None,
      age: None,
    }
  }
}
