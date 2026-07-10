macro_rules! input {
  // create a block (additional curly bracket)
  ($t: ty) => {{
    let mut n = String::new();
    std::io::stdin()
      .read_line(&mut n)
      .expect("failed to read input");

    let n: $t = n.trim().parse().expect("invalid input");
    n
  }};
}

fn main() {
  input!(f32);
}
