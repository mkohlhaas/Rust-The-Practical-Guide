#![allow(unused_imports, unused_variables)]

use rand::random;

// 1.
fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
  if random() { i } else { j } // ⚠️ Error
}

// 2.
// fn picking_int<'a>(i: &'a i32, j: &i32) -> &'a i32 {
//   i
// }

// 3.
// The lifetime of a returned value should be tied to its input parameters.
// When a function returns a reference, that reference must point to something that was provided as an argument.
//
// fn picking_int<'a>(i: &'a i32, j: &i32) -> &'a i32 {
//   let x = 6;
//   &x // ⚠️ Error
// }

// 4.
// static lifetimes
//
// fn picking_int(i: &i32, j: &i32) -> &'static i32 {
//   let y: &'static i32 = &6;
//   y
// }

fn main() {
  let int1 = 5; // int1 lifetime starts 
  let picked_value; // defined here now

  {
    let int2 = 10; // int2 lifetime starts
    picked_value = picking_int(&int1, &int2);
  } // int2 lifetime ends

  println!("{picked_value}");
} // int1 lifetime ends
