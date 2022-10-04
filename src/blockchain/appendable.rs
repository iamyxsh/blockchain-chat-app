use crate::blocks::verifyable::Verifyable;
use crate::{blockchain::Blockchain, blocks::Block};

use super::mintable::Mintable;

pub trait Appendable {
    fn add_block(&mut self, block: Block, wallet: String) -> &Self;
}

impl Appendable for Blockchain {
    fn add_block(&mut self, block: Block, wallet: String) -> &Self {
        if block.verify_block() {
            self.blocks.push(block);

            self.mint(wallet, u64::pow(10, 2));

            self
        } else {
            self
        }
    }
}
