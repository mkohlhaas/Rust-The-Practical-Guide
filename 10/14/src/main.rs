fn main() {
  let int1 = 5; // int1 lifetime starts 
  let picked_value; // defined here now
  {
    let int2 = 10; // int2 lifetime starts
    picked_value = picking_int(&int1, &int2); // Error
  } // int2 lifetime ends
  println!("{picked_value}");
} // int1 lifetime ends
