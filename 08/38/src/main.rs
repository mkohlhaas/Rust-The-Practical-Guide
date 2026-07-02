struct Student {
    name: String,
    age: u8,
    sex: char,
}
fn main() {
    let s_1 = Student {
        name: String::from("ABC"),
        age: 35,
        sex: 'M',
    };

    let s_2 = Student {
        name: String::from("XYZ"),
        age: 35,
        sex: 'M',
    };
}