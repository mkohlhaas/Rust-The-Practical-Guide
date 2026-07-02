macro_rules! some_macro {
  () => {
    x = x + 1; // Error
  };
}
fn main() {
  let mut x = 4;
  some_macro!();
}
