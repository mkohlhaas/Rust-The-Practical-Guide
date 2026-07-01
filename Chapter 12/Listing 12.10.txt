impl Customer {
    ...
    fn new_2(name: String, username: String) -> Self {
        Customer {
            name: name,
            username: username,
            ..Default::default()
        }
    }
}