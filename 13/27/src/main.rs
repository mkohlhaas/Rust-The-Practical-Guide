struct Admin;
struct User;
trait Authenticate {
  fn authenticate(&self, username: &str, password: &str) -> bool;
}
