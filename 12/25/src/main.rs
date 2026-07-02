fn fn3(a: &mut A) {
  let x = fn1(&a.b); // Error
  let y = fn2(&a.c);
  println!("{x}");
}
