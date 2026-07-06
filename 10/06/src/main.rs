fn main() {
  let i;
  {
    let j = 5;
    i = &j;
    println!("i: {i}");
  }
}
