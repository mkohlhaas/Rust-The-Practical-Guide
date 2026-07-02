fn main() {
    	let first_num = 42;
    	let second_num = 64;
    	let ref1 = &mut first_num;
    	let mut ref2 = &mut second_num; 
    	*ref1 = 15;
    	*ref2 = 10;
    	ref2 = ref1;
    	println!("Updated first number: {ref2}");  
}
