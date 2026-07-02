fn main() {
    let new_user = Customer::new("AliceNouman".to_string());
    let’ser_with_login = Customer::new_2("Joseph".to_string(), "joe123".to_string());
    let’ser_with_membership = Customer::new_3(
        "Micheal".to_string(),
        "micheal2000".to_string(),
        Membershiptype::loyal,
    );
}