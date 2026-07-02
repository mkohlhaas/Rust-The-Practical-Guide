fn main() {
    let x = UnSizedStruct { // Error
        sized_field_1: 3,
        unsized_field: [3],
    };
}