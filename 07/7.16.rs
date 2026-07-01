mod shapes {
    ...
    impl Circle {
	  ...
        pub fn contains(&self, other: &Circle) -> bool {
		println!("Checking if the circle is contained");
            self.radius > other.radius
        }
    }
}