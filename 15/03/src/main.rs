macro_rules! our_macro {
  () => {
    1 + 1
  };
}

fn main() {
  // all kinds of braces
  let n1 = our_macro!();
  let n2 = our_macro![];
  let n3 = our_macro! {};

  println!("{n1}");
  println!("{n2}");
  println!("{n3}");
}
