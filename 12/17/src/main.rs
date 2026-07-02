impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            ..Default::default() 
        }
    }
}