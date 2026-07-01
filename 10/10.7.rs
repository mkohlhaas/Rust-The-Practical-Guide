fn main() {
    let mut vec_1 = vec![6, 5, 8, 9];
    let ref_1 = &vec_1; 						// ref_1 lifetime starts
    println!("ref 1: {:?}", ref_1); 					// ref_1 lifetime ends 
    let ref_2 = &mut vec_1; 						// ref_2 lifetime starts 
    ref_2.push(3);
    println!("ref 2: {:?}", ref_2); 					// ref_2 lifetime ends
}