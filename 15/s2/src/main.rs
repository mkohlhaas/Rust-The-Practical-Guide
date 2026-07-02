macro_rules! make_struct {
    ($name:ident {$($field:ident: $ty:ty),*}) => {
        struct $name {
            $($field: $ty),*
        }
    };
}

// Sample usage
make_struct!(MyStruct {
  field1: i32,
  field2: String
});

fn main() {}
