use crate::blockchain::Blockchain;

use super::Wallet;

pub trait Accountable {
    fn get_balance(&self, blockchain: &Blockchain) -> u64;
}

impl Accountable for Wallet {
    fn get_balance(&self, blockchain: &Blockchain) -> u64 {
        let bal = blockchain.balances.get(&self.public_key);

        match bal {
            Some(val) => return val.to_owned(),
            None => return 0,
        }
    }
}
