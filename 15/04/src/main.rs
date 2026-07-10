// The general convention, however, is to use parentheses for the left side of the rule and curly
// brackets for the right side of the rule.

macro_rules! our_macro {
    [] => {   // you can use (), {} or [] for left side of the rule
        1 + 1
    };
    () => [   // you can use (), {} or [] for right side of the rule
        2 + 2
    ];
}

fn main() {
  // all kinds of braces
  let n1 = our_macro!();
  let n2 = our_macro![];
  let n3 = our_macro! {};

  println!("{n1}");
  println!("{n2}");
  println!("{n3}");
}
