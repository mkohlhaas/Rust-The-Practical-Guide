macro_rules! string_concat {
    ($($some_str:expr)*) => {{ // no delimiter => default space delimiter
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
        }
        };
}
fn main() {
  let str_null = string_concat!();
  let str_single = string_concat!("First");
  let str_double = string_concat!("First" "Second"); // Space is used to separate 
  values
}
