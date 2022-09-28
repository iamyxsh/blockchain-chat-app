#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{
        blockchain::{appendable::Appendable, verifyable::Verifyable, Blockchain},
        blocks::Block,
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

        blockchain.add_block(block1.clone());

        assert_eq!(blockchain.blocks.get(1).unwrap(), &block1);
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

        blockchain.add_block(block1.clone());

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

        blockchain.add_block(block2.clone());

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

        blockchain.add_block(block3.clone());

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

        blockchain.add_block(block1.clone());

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

        blockchain.add_block(block2.clone());

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

        blockchain.add_block(block3.clone());

        assert!(!blockchain.verify_chain());
    }
}
