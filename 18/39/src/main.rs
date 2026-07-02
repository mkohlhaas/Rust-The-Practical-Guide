impl WordDictionary {
    ...
    fn search(&self, word: &String) -> bool {
        let mut current = &self.root;
        for w in word.chars() {
            if current.children.get(&w).is_some() {
                current = current.children.get(&w).unwrap();
            } else {
                return false;
            }
        }
        current.is_word
    }
}