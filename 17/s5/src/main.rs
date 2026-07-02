fn main() {
    // Original JSON string with escaped quotes
    let json_data = "{
        \"user\": \"Alice\", 
        \"details\": { 
            \"age\": 29, 
            \"city\": \"Wonderland\" 
        }, 
        \"active\": true
    }";

    // Refactored JSON string using raw string literal
    let json_data_raw = r#"{
        "user": "Alice", 
        "details": { 
            "age": 29, 
            "city": "Wonderland" 
        }, 
        "active": true
    }"#;

    println!("Original: {}", json_data);
    println!("Refactored: {}", json_data_raw);
}