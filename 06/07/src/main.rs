...
mod Order {
    struct Order {
        id: u64,
        product: Product,   // Error	
        customer: Customer, // Error
        quantity: u32,
    }
...
