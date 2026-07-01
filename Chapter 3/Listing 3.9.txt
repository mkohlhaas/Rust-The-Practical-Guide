fn main() {
    let marks = 95;
    let mut grade = 'N';
    match marks {
        90..=100 => grade = 'A',
        80..=90 => grade = 'B',
        70..=79 => grade = 'C',
        _ => grade = 'F',
    }
}