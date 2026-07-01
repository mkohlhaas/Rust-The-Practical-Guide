 fn main() {
    {
	let i = 5;						// i lifetime starts
    } 								// i lifetime ends
    let j = i;
    println!("{i}"); // Error
}