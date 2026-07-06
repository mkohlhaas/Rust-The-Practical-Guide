use rand::random;

// The lifetime of the returned value will be equal to the shorter of the two lifetimes of the parameters i and j.
// For example, if i has a shorter lifetime than j, then the returned reference will have a lifetime tied to i.
// Conversely, if j has the shorter lifetime, the returned reference will be valid for as long as j exists.

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
  if random() { i } else { j }
}

fn main() {
  let int1 = 5;
  let int2 = 10;
  let picked_value: &i32 = picking_int(&int1, &int2);

  println!("{picked_value}");
}
