fn returns_shape() -> Box<dyn Shape> {
    ...
    let x = false;
    if x {
        Box::new(sq)
    } else {
        Box::new(Rect) 
    }
} 