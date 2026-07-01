fn main() {
    let r1 = Rectangle {
        ...
    };
    let s1 = Square {
        ...
    };
    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1)); 
} 