#![allow(dead_code)]

// Unit Structs
// A Unit struct is a struct with no fields.

#[derive(Debug)]
struct Admin;

#[derive(Debug)]
struct User;

trait Authenticate {
  fn authenticate(&self, username: &str, password: &str) -> bool;
}

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

fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
  role.authenticate(username, password)
}

fn main() {
  let admin1 = Admin;
  let user1 = User;

  // there is only one admin and user role in the system
  let admin2 = admin1; // Admin is moved as it is not Copy
  let user2 = user1; // User is moved as it is not Copy

  let admin_login = login(admin2, "admin", "adminpass");
  let user_login = login(user2, "user", "userpass");

  // println!("{:?}", admin); // ⚠️ Error: unit structs like any other struct are not Copy by default; they are moved

  if admin_login {
    println!("Admin login successful!");
  } else {
    println!("Admin login failed!");
  }

  if user_login {
    println!("User login successful!");
  } else {
    println!("User login failed!");
  }
}
