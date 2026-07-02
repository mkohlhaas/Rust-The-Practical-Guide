macro_rules! our_macro {
    [] => {   // you can use (), {} or [] for left side of the rule
        1 + 1
    };

    () => [
        1 + 1
    ]; // you can use (), {} or [] for right side of the rule
}