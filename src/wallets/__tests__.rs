#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use serial_test::serial;

    use crate::{
        blockchain::Blockchain,
        blocks::Block,
        wallets::{accountable::Accountable, signable::Signable, Wallet},
    };

    #[test]
    #[serial]
    fn it_should_create_wallet() {
        let wallet = Wallet::new();

        assert_ne!(wallet.public_key, "");
        assert_ne!(wallet.private_key, "");
    }

    #[test]
    #[serial]
    fn it_should_sign_and_verify() {
        let wallet = Wallet::new();
        let signature = wallet.sign_message("Signature".to_string());

        assert!(wallet.verify_message("Signature".to_string(), &signature));
    }

    #[test]
    #[serial]
    fn it_should_fetch_balance() {
        let wallet = Wallet::new();

        let genesis_block = Block::genesis_block(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let blockchain = Blockchain::start(genesis_block.clone());

        let balance = wallet.get_balance(&blockchain);

        assert_eq!(balance, 0);
    }
}
