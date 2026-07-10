macro_rules! input {
  ($t: ty) => {
    let mut n = String::new(); // Error
    std::io::stdin()
      .read_line(&mut n)
      .expect("failed to read input");

    let n: $t = n.trim().parse().expect("invalid input");
    n
  };
}

fn main() {
  println!("Please enter a floating point number");
  let some_input_0 = input!(f32);
}
