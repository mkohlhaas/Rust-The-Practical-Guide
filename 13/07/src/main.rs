#![allow(dead_code)]
#![feature(negative_impls)]

// opting out of an auto trait

// https://crates.io/crates/negative-impl
use negative_impl::negative_impl;

// ABC is Send and Sync by default
struct ABC;

// we do not allow our type to be transmitted or exchanged between threads
#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}

// #[negative_impl]
// impl !Sized for ABC {} // ⚠️ Error: you can't opt out of Sized

fn main() {}
