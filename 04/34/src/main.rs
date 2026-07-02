#![allow(unused_variables, unused_assignments)]

fn main() {
  let vec1 = vec![1, 2, 3];
  let vec2 = vec![4, 5, 6];

  let mut reference = &mut &vec1;
  println!("{:?}", reference);

  // https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211/2
  // reference = &mut &vec2; // consider using a `let` binding to create a longer lived value
  let mut binding = &vec2;
  reference = &mut binding;

  println!("{:?}", reference)
}
