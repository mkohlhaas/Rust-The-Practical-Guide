fn fn1(a: &mut B) -> &u32 {
    &a.f2
}
fn fn2(a: &mut C) -> u32 {
    a.f1 + a.f3
}