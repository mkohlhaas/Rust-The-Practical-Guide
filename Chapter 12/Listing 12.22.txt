fn fn1(a: &A) -> &u32 {
    &a.f2
}
fn fn2(a: &A) -> u32 {
    a.f1 + a.f3
}
fn fn3(a: &mut A) {
    let x = fn1(a);
    let y = fn2(a);
    println!("{x}");
}