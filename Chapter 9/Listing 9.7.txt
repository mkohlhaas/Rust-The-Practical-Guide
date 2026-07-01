fn main() {
    ...
    let mut banned_user = String::from("banned user");
    let validate_user_simple = |name: &str| {
        let banned_user_name = banned_user;
        name.len() != 0 && name != banned_user_name
    };
    ...	
}