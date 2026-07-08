#![allow(clippy::let_unit_value, unused_variables)]

// `Unit structs` are not Copy, they are moved.
struct Abc;

fn main() {
  println!("{}", size_of::<Abc>()); // 0

  {
    let a = ();
    let b = a; // unit type can be copied many types
    let c = a;
  }

  // {
  //   let x = Abc; // unit struct cannot copied many times
  //   let y = x;
  //   let z = x; // ⚠️ Error: use of moved value: `x`
  // }
}
