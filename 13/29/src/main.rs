fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
    role.authenticate(username, password)
}