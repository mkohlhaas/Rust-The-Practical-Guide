mod m1 {
    struct A {
        d: m2::D,
    }
    mod m2 {
        enum D {
            B,
            C,
        }
    }
}

fn main(){}