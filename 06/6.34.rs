pub use customer::Customer; 
pub use product::{category::Category,Product};  
mod product { 
    pub use category::Category;	
    ...
}
mod customer { 
    ...
}
mod order {
    ...
}