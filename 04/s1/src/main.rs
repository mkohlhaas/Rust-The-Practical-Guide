fn main() {
  let s1: String = String::from("this is me, ");
  let s2: &str = "Nouman";
  some_function(&s1, s2);
  println!("{} {}", s1, s2);
}

fn some_function(a1: &String, a2: &str) {
  // update the first input to a reference
  // String
  println!("{} {}", a1, a2);
}
