impl Customer {
    ...
    fn new_3(name: String, username: String, membership: Membershiptype) -> Self {
        Customer {
            name: name,
            username: username,
            membership: membership,
            ..Default::default()
        }
    }
}