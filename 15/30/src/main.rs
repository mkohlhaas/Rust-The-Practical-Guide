fn main() {
    let str_null = String::new();
    let str_single = {
        let mut temp_str = String::new();
        temp_str.push_str("First");
        temp_str
    };
    let str_double = {
        let mut temp_str = String::new();
        temp_str.push_str("First");
        temp_str.push_str("Second");
        temp_str
    };
}