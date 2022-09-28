use serde::{Deserialize, Serialize};

use crate::blocks::Block;

mod __tests__;
pub mod appendable;
pub mod verifyable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn start(genesis_block: Block) -> Self {
        let mut blockchain = Blockchain { blocks: Vec::new() };

        blockchain.blocks.push(genesis_block);

        blockchain
    }
}
