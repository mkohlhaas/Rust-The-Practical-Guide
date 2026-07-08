#![allow(dead_code)]

use std::fmt::Debug;

// Rust uses pattern matching when resolving the actual concrete types that need to be substituted for T:
//
// | --------------------+-------------+---------+---------- |
// | Parameter Type      | T (1)       | &T (2)  | &T        |
// | --------------------+-------------+---------+---------- |
// | Function Call Input | &str        | &str    | &&str     |
// | Resolves To         | T = &str    | T = str | T = &str  |
// | --------------------+-------------+---------+---------- |
//
// By default Rust wants to be the resulting type T as Sized.

// (1)
fn print_fn_by_val<T: Debug>(t: T) {
  println!("{:?}", t);
}

// (2)
fn print_fn_by_ref<T: Debug>(t: &T) {
  println!("{:?}", t);
}

fn print_fn_trait_bound<T: Debug + ?Sized>(t: &T) {
  println!("{:?}", t);
}

fn main() {
  let x: &str = "my name";

  {
    // (1)
    print_fn_by_val(x);
  }

  {
    // (2)
    // print_fn_by_ref(x); // ⚠️ Error: the size for values of type `str` cannot be known at compilation time
  }

  {
    // (3)
    print_fn_by_ref(&x);
  }

  {
    print_fn_trait_bound(x);
    print_fn_trait_bound(&x);
  }
}
