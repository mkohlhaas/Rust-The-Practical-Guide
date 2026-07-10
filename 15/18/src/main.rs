macro_rules! add_as {
  ($a: expr, $b: expr, $typ: ty) => {
    $a as $typ + $b as $typ
  };
}

fn main() {
  let res = add_as!(1 + 1, 2 * 2, u16);
  println!("{res}");
}
