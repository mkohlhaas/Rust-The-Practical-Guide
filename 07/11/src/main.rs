mod shapes {
...
    impl Circle {
	 ...
        pub fn new_2(radius: f32) -> Circle {
            match radius {
                ..=0.0 => panic!("radius should be positive"),
                _ => Circle { radius },
            }
        }
     }
}