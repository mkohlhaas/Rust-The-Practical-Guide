// Captures

macro_rules! our_macro {
  () => {
    1 + 1
  };

  ($e1:expr, $e2:expr) => {
    $e1 + $e2
  };
}

fn main() {
  println!("{}", our_macro!(2, 2));
}
