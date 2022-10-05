use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::blocks::Block;

mod __tests__;
pub mod appendable;
pub mod mintable;
pub mod savable;
pub mod verifyable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub balances: HashMap<String, u64>,
    pub current_difficulty: u16,
}

impl Blockchain {
    pub fn start(genesis_block: Block) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            balances: HashMap::new(),
            current_difficulty: 1,
        };

        blockchain.blocks.push(genesis_block);

        blockchain
    }
}
