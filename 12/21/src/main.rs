fn fn3(a: &mut A) {
  let x = fn1(a);
  let y = fn2(a);
  println!("{x}"); // Error
}
