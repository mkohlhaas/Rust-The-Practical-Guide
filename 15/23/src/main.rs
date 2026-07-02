macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1; // Error
    };
}
fn main() {
    let mut x = 4;
    some_macro!(x);
}