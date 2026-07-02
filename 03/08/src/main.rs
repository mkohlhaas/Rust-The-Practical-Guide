fn main() {
  let marks = 95;
  let grade;

  match marks {
    90..=100 => grade = 'A',
    80..=89 => grade = 'B',
    _ => grade = 'F',
    70..=79 => grade = 'C', // ⚠️ unreachable code
  }

  println!("Grade: {grade}")
}
