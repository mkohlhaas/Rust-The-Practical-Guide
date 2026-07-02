macro_rules! our_macro {
  () => {
    1 + 1;
  };
  (something@_@) => {
    println!("You found nonsense here")
  };
}
fn main() {
  our_macro!(something@_@);
}
