#![allow(unused_variables)]

// The never type is indicated by the exclamation point (!).

// The never type represents computations that never resolve to any value.
// In other words, these computations will always panic or will always exit the program.

fn main() {
  let n: i32 = match "123".parse::<i32>() {
    Ok(num) => num,
    Err(_) => panic!(), // the never type can be coerced to any other type
  };

  println!("{n}");

  // let x: ! = panic!(); // ⚠️ panic returns the never type

  // let x: ! = return; // ⚠️ return also results in a never type

  let x: String = return; // never type can be coerced to any type
}
