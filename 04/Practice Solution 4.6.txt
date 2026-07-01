fn main() {
    let mut first_num = 42; // We are using mutable references to it so the variable  
                               must be mutable
    let mut second_num = 64; // We are using mutable references to it so the variable 
                                must be mutable
    let ref1 = &mut first_num;
    let mut ref2 = &mut second_num; // a mutable references means that the reference 
                                       can be updated to point to some other variable

    *ref1 = 15;
    *ref2 = 10;
    ref2 = ref1;
    println!("Updated first number: {ref2}");  
}