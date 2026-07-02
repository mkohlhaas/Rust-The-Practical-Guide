fn some_if_greater(number: &i32, greater_than: &i32) -> Option<&i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}
fn main() {
    let num_1 = 7;
    let greater_val = 4;
    let test = some_if_greater(&num_1, &greater_val);
}