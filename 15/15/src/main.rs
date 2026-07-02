let some_input_0 = 
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

   let n: $t = n.trim().parse().expect("invalid input");
    n