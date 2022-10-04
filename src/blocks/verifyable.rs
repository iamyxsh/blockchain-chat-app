use std::iter::repeat;

use super::Block;

pub trait Verifyable {
    fn verify_block(&self) -> bool;
}

impl Verifyable for Block {
    fn verify_block(&self) -> bool {
        self.clone().hash.unwrap().starts_with(
            repeat("0")
                .take(self.difficulty.unwrap().into())
                .collect::<String>()
                .as_str(),
        )
    }
}
