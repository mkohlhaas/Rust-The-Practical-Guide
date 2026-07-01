macro_rules! some_macro {
    () => {
        let mut x = 4;
    };
}  
fn main() {
    some_macro!();
    x = x+1; // Error
}