fn add(x: u32, y: u32) -> u32 {
    x + y
}
fn square(x: u32) -> u32 {
    x * x
}
fn sum_of_squares(num: ?, sq: ?, add: ?) -> u32 { 
    let mut result = 0;
    for i in 1..=num {
        result = add(result, sq(i));
    }
    result
}
fn main() {
    let num = 4;
    let sum = sum_of_squares(num, square, add);
    println!("Sum of squares from 1 to {} = {}", num, sum);
}