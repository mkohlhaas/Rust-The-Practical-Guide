// Implementing basic arithmetic functions

fn times(n1: i32, n2: i32) -> i32 {
  n1 * n2
}

fn add_3(n: i32) -> i32 {
  n + 3
}

fn add_5(n: i32) -> i32 {
  n + 5
}

fn main() {
  let x = 3;
  let y = 4;

  println!(
    "The result of x+3 times y+5 is {}",
    times(add_3(x), add_5(y))
  );
}
