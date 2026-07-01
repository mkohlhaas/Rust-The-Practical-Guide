fn main() {
    let int1 = 5; 						// int1 lifetime starts
    let int2 = 10; 						// int2 lifetime starts
    let picked_value = picking_int(&int1, &int2);
    println!("{picked_value}");
} 								// int1 and int2 lifetime ends