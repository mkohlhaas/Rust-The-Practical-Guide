fn main() {
    let int1 = 5;
    let int2 = 10;
    let picked_value = picking_int(&int1, &int2);
    println!("{picked_value}");
}
fn picking_int(i: &i32, j: &i32) -> i32 {
    if rand::random() {
        *i
    } else {
        *j
    }
}