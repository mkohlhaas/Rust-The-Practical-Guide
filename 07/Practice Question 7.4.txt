mod University {
    pub struct Student {
        name: String,
        marks: u8,
        grade: char,
    }
}
use University::Student;
fn main() {
    let mut student_1 = Student {
        name: String::from("Alice"),
        marks: 75,
        grade: 'A',
    };
    println!("{} got {} grade", student_1.name, student_1.grade);
}