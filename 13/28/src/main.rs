impl Authenticate for Admin {
  fn authenticate(&self, username: &str, password: &str) -> bool {
    username == "admin" && password == "adminpass"
  }
}

impl Authenticate for User {
  fn authenticate(&self, username: &str, password: &str) -> bool {
    username == "user" && password == "userpass"
  }
}
