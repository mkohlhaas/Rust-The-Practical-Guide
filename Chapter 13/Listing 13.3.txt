trait Some_trait {} 
fn main() {
        println!(
        "The size of the trait object is: {}",
        size_of::<dyn Some_trait>() 
    ); // Error
}