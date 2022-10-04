#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        blockchain::{
            appendable::Appendable, mintable::Mintable, verifyable::Verifyable, Blockchain,
        },
        blocks::{verifyable::Verifyable as BlockVerify, Block},
        wallets::{accountable::Accountable, Wallet},
    };

    #[test]
    fn it_should_add_genesis_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let blockchain = Blockchain::start(genesis_block.clone());

        let block1 = blockchain.blocks.get(0).unwrap();

        assert_eq!(&genesis_block, block1);
    }

    #[test]
    fn it_should_add_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let mut block: Block = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            0,
            vec![0; 32],
            genesis_block.clone().hash.unwrap(),
        );

        for x in 0..u128::MAX {
            let block1 = Block::new(
                "party_one".to_string(),
                "party_two".to_string(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                x,
                vec![0; 32],
                genesis_block.clone().hash.unwrap(),
            );

            if block1.verify_block() {
                block = block1.clone();
                blockchain.add_block(block1.clone(), "wallet".to_string());
            }
        }

        assert_eq!(
            blockchain.blocks.get(1).unwrap().clone().hash.unwrap(),
            block.clone().hash.unwrap()
        );
    }

    #[test]
    fn it_can_verify_chain() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let block1 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            genesis_block.hash.unwrap(),
        );

        blockchain.add_block(block1.clone(), "wallet".to_string());

        let block2 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            101,
            vec![0; 32],
            block1.hash.unwrap(),
        );

        blockchain.add_block(block2.clone(), "wallet".to_string());

        let block3 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            101,
            vec![0; 32],
            block2.hash.unwrap(),
        );

        blockchain.add_block(block3.clone(), "wallet".to_string());

        assert!(blockchain.verify_chain());
    }

    #[test]
    fn it_can_detect_faulty_chain() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let block1 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            genesis_block.hash.unwrap(),
        );

        blockchain.add_block(block1.clone(), "wallet".to_string());

        let block2 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            101,
            vec![0; 32],
            block1.clone().hash.unwrap(), //added a faulty hash over here
        );

        blockchain.add_block(block2.clone(), "wallet".to_string());

        let block3 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            101,
            vec![0; 32],
            block1.hash.unwrap(),
        );

        blockchain.add_block(block3.clone(), "wallet".to_string());

        assert!(!blockchain.verify_chain());
    }

    #[test]
    fn it_can_mint_to_wallet() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let wallet = Wallet::new();

        blockchain.mint(wallet.public_key.clone(), u64::pow(10, 2));

        let bal = wallet.get_balance(&blockchain);

        assert_eq!(bal, u64::pow(10, 2));
    }

    #[test]
    fn it_can_reward_for_adding_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let wallet = Wallet::new();

        let block1 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            genesis_block.hash.unwrap(),
        );

        blockchain.add_block(block1.clone(), wallet.public_key.clone());

        let bal = wallet.get_balance(&blockchain);

        assert_eq!(bal, u64::pow(10, 2));

        let block2 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            block1.hash.unwrap(),
        );

        blockchain.add_block(block2.clone(), wallet.public_key.clone());

        let bal2 = wallet.get_balance(&blockchain);

        assert_eq!(bal2, 2 * u64::pow(10, 2));
    }

    #[test]
    fn it_can_verify_block_before_adding() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut blockchain = Blockchain::start(genesis_block.clone());

        let wallet = Wallet::new();

        let block1 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            genesis_block.hash.unwrap(),
        );

        blockchain.add_block(block1.clone(), wallet.public_key.clone());

        let bal = wallet.get_balance(&blockchain);

        assert_eq!(bal, u64::pow(10, 2));

        let block2 = Block::new(
            "party_one".to_string(),
            "party_two".to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            100,
            vec![0; 32],
            block1.hash.unwrap(),
        );

        blockchain.add_block(block2.clone(), wallet.public_key.clone());

        let bal2 = wallet.get_balance(&blockchain);

        assert_eq!(bal2, 2 * u64::pow(10, 2));
    }
}
