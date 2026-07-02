macro_rules! vec_mac {
        ( $($element: expr), *) => {{
            let mut some_vec = Vec::new();
            $(some_vec.push($element);)*
            some_vec
        }
    };
}