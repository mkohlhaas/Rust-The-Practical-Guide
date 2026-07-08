fn main() {
  let mut counter = 0;

  // `break` and `continue` return the never type.
  // For historical reasons result is coerced to the unit type.
  let result: () = loop {
    counter += 1;
    if counter == 10 {
      break;
    }
  };

  println!("{:?}", result)
}
