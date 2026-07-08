fn main() {
  let admin = Admin;
  let user = User;

  let admin_login = login(admin, "admin", "adminpass");
  let user_login = login(user, "user", "userpass");

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

