let mut banned_user = String::from("banned user");
let validate_user_simple = move |name: &str| { // Error 
    let banned_user_name = &banned_user;
    name.len() != 0 && name != banned_user_name
}; 
println!("{banned_user}"); // Error 