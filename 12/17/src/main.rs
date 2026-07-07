#![allow(clippy::new_ret_no_self, dead_code, unused_variables)]

#[derive(Debug, Default, Clone)]
struct Customer {
  name: String,
  username: String,
  membership: Membershiptype,
  gender: char,
  country: String,
  age: u8,
}

impl Customer {
  fn new(name: String) -> CustomerBuilder {
    CustomerBuilder {
      name,
      ..Default::default()
    }
  }
}

#[derive(Default)]
struct CustomerBuilder {
  name: String,
  username: Option<String>,
  membership: Option<Membershiptype>,
  gender: Option<char>,
  country: Option<String>,
  age: Option<u8>,
}

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

  fn build(&mut self) -> Customer {
    Customer {
      name: self.name.clone(),
      username: self.username.clone().unwrap_or_default(),
      membership: self.membership.clone().unwrap_or_default(),
      gender: self.gender.unwrap_or_default(),
      country: self.country.clone().unwrap_or_default(),
      age: self.age.unwrap_or_default(),
    }
  }
}

#[derive(Debug, Clone, Default)]
enum Membershiptype {
  #[default]
  New,
  Causual,
  Loyal,
}

// impl Default for Membershiptype {
//   fn default() -> Self {
//     Membershiptype::New
//   }
// }

fn main() {}
