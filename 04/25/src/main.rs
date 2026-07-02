fn main(){}
fn gives_ownership() -> &Vec<i32> { // Error
    	let vec = vec![4, 5, 6];	
	&vec
}
