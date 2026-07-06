fn main() {
  let i: &i32;

  {
    let j = 5;
    i = &j;
    println!("i: {i}");
  }
}
