use serde::{Serialize, Deserialize};
use sha256::digest;

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct Hash(String);

impl Hash {
    pub fn new(hash: &str) -> Hash {
        Hash(digest(hash))
    }

    pub fn from(vec: &[u8]) -> Hash {
        Hash(digest(vec))
    }

    pub fn from_hashed(hash: &str) -> Hash {
        Hash(String::from(hash))
    }

    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

pub trait DVCSHash {
    fn get_hash(&self) -> Hash;
}