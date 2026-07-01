fn main() {
    let words = vec![
        "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
    ].into_iter().map(|x| String::from(x)).collect::<Vec<String>>();

    let mut d = WordDictionary::new();

    for i in 0..words.len() {
        d.insert(&words[i]);
    }
    println!(
        "Word 'there' in the dictionary: {}",
        d.search(&"there".to_string())
    );
}