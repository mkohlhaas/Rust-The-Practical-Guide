impl CustomerBuilder {
  fn username(&mut self, username: String) -> &mut Self {
    self.username = Some(username);
    self
  }

  fn membership(&mut self, membership: Membershiptype) -> &mut Self {
    self.membership = Some(membership);
    self
  }

  fn gender(&mut self, gender: char) -> &mut Self {
    self.gender = Some(gender);
    self
  }
  fn country(&mut self, country: String) -> &mut Self {
    self.country = Some(country);
    self
  }
  fn age(&mut self, age: u8) -> &mut Self {
    self.age = Some(age);
    self
  }
}
