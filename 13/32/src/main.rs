struct ABC;
use negative_impl::negative_impl;
#[negative_impl]
impl !Send for ABC {}

#[negative_impl]
impl !Sync for ABC {}