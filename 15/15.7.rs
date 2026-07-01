macro_rules! our_macro {
    ...

    ($e1:expr, $e2:expr) => {
        $e1 + $e2
    };
}
fn main() {
    println!("{}", our_macro!(2, 2));
}