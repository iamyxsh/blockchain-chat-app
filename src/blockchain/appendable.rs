use crate::{blockchain::Blockchain, blocks::Block};

pub trait Appendable {
    fn add_block(&mut self, block: Block) -> &Self;
}

impl Appendable for Blockchain {
    fn add_block(&mut self, block: Block) -> &Self {
        self.blocks.push(block);

        self
    }
}
