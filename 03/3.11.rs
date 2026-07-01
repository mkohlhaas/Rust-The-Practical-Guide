fn main() {
    'outer: loop {
        'inner: loop {
            println!("Simple loop");
            break 'outer;
        }
    }
}