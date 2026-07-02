fn main() {
  let marks = 95;
  let grade;

  match marks {
    90..=100 => grade = 'A',
    80..=89 => grade = 'B',
    70..=79 => grade = 'C',
    _ => grade = 'F',
  }

  println!("Grade: {grade}")
}
