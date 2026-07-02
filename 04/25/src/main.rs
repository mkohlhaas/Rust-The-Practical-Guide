// ⚠️ Error
fn gives_ownership() -> &Vec<i32> {
  let vec = vec![4, 5, 6];
  &vec
}

fn main() {}
