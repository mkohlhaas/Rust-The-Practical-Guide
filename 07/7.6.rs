mod shapes {
    ...
    impl Circle {
       ...
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius < other.radius // Condition changed
        }
    }
}
...