fn picking_int(i: &i32, j: &i32) -> &i32 {
  // Error
  if rand::random() { i } else { j }
}
