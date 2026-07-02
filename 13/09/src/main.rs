struct UnSizedStruct {
    sized_field_1: i32,
    unsized_field: [i32], // Error
    sized_field_2: i32,
}