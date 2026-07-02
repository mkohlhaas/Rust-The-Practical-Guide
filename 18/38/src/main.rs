impl WordDictionary {
    ...
    fn insert(&mut self, word: &String) {
        let mut current = &mut self.root;
        for w in word.chars() {
            current = current.children.entry(w).or_insert(Node::new());
        }

        if !current.is_word {
            current.is_word = true;
        }
    }
}