fn fn3(a: &mut A) {
  let x = fn1(&mut a.b);
  let y = fn2(&mut a.c);
  println!("{x}");
}
