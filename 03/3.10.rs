fn main() {
    let marks = 95;
    let mut grade = match marks {
        90..=100 => 'A',
        80..=90 => 'B',
        70..=79 => 'C',
        _ => 'F',
    };
}