use std::hash::Hash;
use std::error::Error;


pub trait StateObject {
    type StateResult: Into<Vec<u8>>;
    type Address: Hash;
    type StateError: Error;

    fn get_code(&self, addr: &Self::Address) -> Result<Self::StateResult, Self::StateError>;
}

