macro_rules! add_as {
  ($a: expr, $b: expr, $typ: ty) => {
    $a as $typ + $b as $typ
  };
}

fn main() {
  println!("{}", add_as!(15, 2.3, f32));
  println!("{}", add_as!(15, 2.3, f32));
}
