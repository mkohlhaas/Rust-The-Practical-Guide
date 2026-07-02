pub use customer::Customer; 
pub use product::{category::Category,Product};  // Error
mod product { 
    ...
}
mod customer { 
    ...
}
mod order {
    ...
}