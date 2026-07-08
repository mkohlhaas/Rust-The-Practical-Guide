#![allow(dead_code, unused_variables)]

fn some_fn1<T>(t: T) {}

fn some_fn2<T: Sized>(t: T) {} // NOTE: `Sized` is the default trait bound

// Rust needs to know the sizes for variables at compile time
// fn some_fn3<T: ?Sized>(t: T) {} // ⚠️ Error

// `t` is a reference to a possibly unsized type
fn some_fn4<T: ?Sized>(t: &T) {}

fn main() {
  {
    println!(
      "Size of a reference to sized type: {}", // 8
      size_of::<&[i32; 3]>()
    );

    println!(
      "Size of a reference to unsized type: {}", // 16
      size_of::<&[i32]>()
    );
  }

  println!();

  {}
}
