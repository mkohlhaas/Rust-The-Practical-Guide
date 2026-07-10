macro_rules! our_macro {
  () => {
    1 + 1
  };

  ($a:expr, $b:expr, $c:expr) => {
    $a * ($b + $c)
  };
}

fn main() {
  println!("{}", our_macro!(5, 6, 3));
}
