use self::hashable::Hashable;
use serde::{Deserialize, Serialize};

mod __tests__;
mod hashable;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub party_one: String,
    pub party_two: String,
    pub timestamp: u64,
    pub difficulty: Option<u8>,
    pub nonce: u128,
    pub payload: Vec<i128>,
    pub hash: Option<String>,
    pub previous_hash: Option<String>,
}

impl Block {
    pub fn new(
        party_one: String,
        party_two: String,
        timestamp: u64,
        nonce: u128,
        payload: Vec<i128>,
    ) -> Self {
        let mut block = Block {
            party_one,
            party_two,
            timestamp,
            nonce,
            payload,
            difficulty: None,
            hash: None,
            previous_hash: None,
        };

        block.hash = Some(block.gen_hash());

        block
    }
}
