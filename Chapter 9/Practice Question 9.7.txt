struct Person {
    name: String,
    age: u32,
    occupation: String,
}
impl IntoIterator for Person {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        /* Your code here 
        Hint: Should return a vector of Strings, 
        representing the fields of the struct  */
    }
}
fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        occupation: "Software Engineer".to_string(),
    };
    let mut person_iterator = person.into_iter();
    while let Some(property) = person_iterator.next() {
        println!("{}", property);
    }
}