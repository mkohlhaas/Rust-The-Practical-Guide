macro_rules! string_concat {
    ($($some_str:expr),*) => {{
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
    }};
}
