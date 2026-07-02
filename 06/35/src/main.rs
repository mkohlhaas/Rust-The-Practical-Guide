pub use customer::Customer; 
pub use product::Product;  
pub use crate::product::Category; // this syntax will work on newer versions
mod product { 
    pub use category::Category;	
    ...
}
...