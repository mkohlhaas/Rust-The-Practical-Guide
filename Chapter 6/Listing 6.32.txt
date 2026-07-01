pub use customer::Customer; 
pub use product::Product; 
mod product { // not needed to be pub anymore
    ...
}
mod customer { // not needed to be pub anymore
    ...
}
mod order {
    ...
}