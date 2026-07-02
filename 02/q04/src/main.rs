// Correctly shadowing a variable

#![allow(unused_variables)]

fn main() {
  let a = "three"; // don't change this line
  let a = 10; // don't change the name of this variable
  println!("a is: {}", a);
}
