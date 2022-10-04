use crate::blockchain::Blockchain;

pub trait Mintable {
    fn mint(&mut self, wallet: String, amount: u64) -> &Self;
}

impl Mintable for Blockchain {
    fn mint(&mut self, wallet: String, amount: u64) -> &Self {
        *self.balances.entry(wallet).or_insert(0) += amount;

        self
    }
}
