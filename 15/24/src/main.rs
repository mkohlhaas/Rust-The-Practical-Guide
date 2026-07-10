// The stringify macro converts the input passed into its respective String.

macro_rules! create_function {
  ($func_name:ident, $input: ident, $type_input: ty, $type_output: ty) => {
    fn $func_name($input: $type_input) -> $type_output {
      println!(
        "You called {:?}() with the input of {:?}",
        stringify!($func_name),
        stringify!($input)
      );
      $input
    }
  };
}

create_function!(f1, x, i32, i32);

fn main() {
  println!("Function f1 should return: {}", f1(15));
}
