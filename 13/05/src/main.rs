fn main() {
  let num1: &[i32; 3] = &[1, 2, 3];
  let num2: &[i32] = &[4, 5, 6];

  println!("Size of num1: {}", size_of::<&[i32; 3]>()); // 8 (pointer)
  println!("Size of num2: {}", size_of::<&[i32]>()); // 16 (pointer + length)

  let mut sum = 0;

  // thin pointer
  // Rust knows the length of num1 (embedded in the type)
  for num in num1 {
    sum += num;
  }

  // fat pointer
  // Rust knows the length of num2
  for num in num2 {
    sum += num;
  }

  println!("{sum}");
}

