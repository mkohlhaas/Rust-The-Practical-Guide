mod shapes {
    ...
    impl Circle {
	...
      pub fn new_2(radius: f32) -> Circle {
            match radius {
                ..=-10.0 => panic!("is less than -10.0"),
                -10.0..=0.0 => panic!("is between -10.0 and 0.0"),
                _ => Circle { radius },
            }
        }
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}