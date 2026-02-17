use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Validator {
    pub public_key: Vec<u8>,
    pub stake: u64,
    pub is_slashed: bool,
}

impl Validator {
    pub fn new(public_key: Vec<u8>, stake: u64) -> Self {
        Self {
            public_key,
            stake,
            is_slashed: false,
        }
    }
}
