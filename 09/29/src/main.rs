let result: Vec<String> = words
    .into_iter() 
    .filter(|&word| word.starts_with("a") || word.starts_with("b")) 
    .map(|word| word.to_uppercase()) 