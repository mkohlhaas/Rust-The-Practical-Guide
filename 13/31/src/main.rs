struct ABC;
fn main() {
    let a = ();
    let b = a;
    let c = a;
    let x = ABC;
    let y = x;
    let z = x; // Error
}