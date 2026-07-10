// Macro Hygiene & Identifiers Capture
//
// Identifiers come into play because they allow you to cross boundaries or scopes that might otherwise not be possible.

macro_rules! some_macro {
  ($var: ident) => {
    $var = $var + 1;
  };
}

fn main() {
  let mut x = 4;
  some_macro!(x);

  println!("{x}");
}
