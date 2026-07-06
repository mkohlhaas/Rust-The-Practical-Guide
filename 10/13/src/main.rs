use rand::random;

fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
  if random() { i } else { j }
}

fn main() {
  let int1 = 5; // int1 lifetime starts

  {
    let int2 = 10; // int2 lifetime starts
    let picked_value = picking_int(&int1, &int2);

    println!("{picked_value}");
  } // int2 lifetime ends
} // int1 lifetime ends
