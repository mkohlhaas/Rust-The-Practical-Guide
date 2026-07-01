fn print_fn<T: Debug + ?Sized>(t: &T) {
    println!("{:?}", t);
}