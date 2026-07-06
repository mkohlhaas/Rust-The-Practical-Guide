// stack-allocated data

fn main() {
  {
    let i = 5; // i lifetime starts
  } // i lifetime ends

  let j = i; // ⚠️ Error

  println!("{i}"); // Error
}
