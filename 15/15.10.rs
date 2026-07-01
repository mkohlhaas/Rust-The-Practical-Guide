macro_rules! our_macro {
    ...
    ($a:expr, $b:expr; $c:expr) => {
        $a * ($b + $c)
    };
}
fn main() {
    println!("{}", our_macro!(5, 6; 3)); // fixed by changing comma to semicolon
}