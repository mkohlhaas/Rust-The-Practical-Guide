fn main() {
  let marks = 95;
  let grade = match marks {
    90..=100 => 'A',
    80..=90 => 'B', // ⚠️ overlapping ranges
    70..=79 => 'C',
    _ => 'F',
  };

  println!("Grade: {grade}")
}
