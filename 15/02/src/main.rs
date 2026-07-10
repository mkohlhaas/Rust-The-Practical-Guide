macro_rules! our_macro {
  () => {
    1 + 1
  };
}

fn main() {
  let n = our_macro!();

  println!("{n}");
}
