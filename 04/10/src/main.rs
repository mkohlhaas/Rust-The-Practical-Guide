fn main() {
	let mut vec_1 = vec![4, 5, 6];
    	let ref1 = &vec_1;
    	let ref2 = &vec_1;  
	println!(“ref1: {:?}, ref2: {:?}”, ref1, ref2);
}
