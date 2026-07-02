fn main() {
    ...
    let input_word = String::from("teh");
    let grouping = word_grouping(words);

    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The group of the word is {:?}", i);
        }
    }
}