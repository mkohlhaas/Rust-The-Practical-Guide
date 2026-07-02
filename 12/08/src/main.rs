#[derive(Debug, Default, Clone)]
struct Customer {
  name: String,
  username: String,
  membership: Membershiptype,
  gender: char,
  country: String,
  age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
  new,
  causual,
  loyal,
}

impl Default for Membershiptype {
  fn default() -> Self {
    Membershiptype::new
  }
}
