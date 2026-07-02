macro_rules! add_as {
  ($a: expr, $b: expr, $typ: ty) => {
    $a as $typ + $b as $typ
  };
}
