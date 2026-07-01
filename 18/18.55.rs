impl DisplayLobby {
    ...
    fn next_page(&mut self) -> Vec<String> {
        let mut resultant_names: Vec<String> = Vec::new();
        for i in 0..10 {
            if !self.stack.is_empty() {
                resultant_names.push(self.next_name());
            } else {
                break;
            }
        }
        resultant_names
    }
}