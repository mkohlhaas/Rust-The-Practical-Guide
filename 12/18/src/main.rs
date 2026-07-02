fn main() {
        ...
    let’ser_with_login = Customer::new("Joseph".to_string())
        .username("joe123".to_string())
        .build();

    let’ser_with_membership = Customer::new("Micheal".to_string())
        .username("micheal2000".to_string())
        .membership(Membershiptype::loyal)
        .build();
}