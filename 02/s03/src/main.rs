fn main() {
  let mut x1 = 40;
  let x2;
  x1 = x1 * 3;
  x2 = x1 - 2;
  println!("x1 is: {}, x2 is: {}", x1, x2);
}
// Explanation:
// Yes, it will compile. Although the variable x2 is not mutable and we are trying to assign
// a value to this variable in the line "x2 = x1 - 2" but it is ok since the immutable
// variables can be assigned values once.
// Since we are not updating its value later on therefore the variable x2 is assigned
// a value once which is consistent with the definition of immutable variables,
// i.e., they can only be assigned value once.
