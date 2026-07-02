fn main() {
  let marks = 95;

  let grade = if marks >= 90 {
    'A'
  } else if marks >= 80 {
    'B'
  } else if marks >= 70 {
    'C'
  } else {
    'F'
  };

  println!("Grade: {grade}")
}
