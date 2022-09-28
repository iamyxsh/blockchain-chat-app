use crate::blockchain::Blockchain;

pub trait Verifyable {
    fn verify_chain(&self) -> bool;
}

impl Verifyable for Blockchain {
    fn verify_chain(&self) -> bool {
        let mut verify = true;

        for (i, block) in self.blocks.iter().enumerate() {
            if self.blocks.get(i + 1).is_some() && i != 0 {
                if self.blocks.get(i + 1).unwrap().previous_hash == block.hash {
                    continue;
                } else {
                    verify = false;
                    break;
                }
            }
        }

        verify
    }
}
