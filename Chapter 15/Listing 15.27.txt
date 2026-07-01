macro_rules! string_concat {
    () => {
        String::new();
    };

    ($some_str: expr) => {{
        let mut temp_str = String::new();
        temp_str.push_str($some_str);
        temp_str
    }};

    ($some_s1: expr, $some_s2:expr) => {{
        let mut temp_str = String::new();
        temp_str.push_str($some_s1);
        temp_str.push_str($some_s2);
        temp_str
    }};
}