#[derive(Default)]
struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}