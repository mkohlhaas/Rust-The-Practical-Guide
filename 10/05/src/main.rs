fn main() {
  let i: &i32;

  {
    let j = 5; // j lifetime starts from this line
    i = &j; // ⚠️ Error
  } // j lifetime ends on this line

  println!("i: {i}");
}
