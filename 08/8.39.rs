impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}