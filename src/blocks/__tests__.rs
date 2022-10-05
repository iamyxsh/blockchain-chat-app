#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::{
        blockchain::{appendable::Appendable, Blockchain},
        blocks::{verifyable::Verifyable, Block},
        utils::{hashing::hash_string, json::ser_json},
    };
    use std::{
        iter::repeat,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    #[serial]
    fn it_should_init_genesis_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let block = Block::genesis_block(now);

        let hash = hash_string(ser_json(&Block {
            timestamp: now,
            difficulty: Some(1),
            hash: None,
            previous_hash: None,
            party_one: "PARTY_ONE".to_string(),
            party_two: "PARTY_TWO".to_string(),
            nonce: 100,
            payload: vec![0; 32],
        }));

        assert_eq!(block.party_one, "PARTY_ONE".to_string());
        assert_eq!(block.party_two, "PARTY_TWO".to_string());
        assert_eq!(block.timestamp, now);
        assert_eq!(block.nonce, 100);
        assert_eq!(block.payload, vec![0; 32]);
        assert_eq!(block.hash.unwrap(), hash);
        assert_eq!(block.previous_hash, None);
    }

    #[test]
    fn it_should_verify_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let genesis_block = Block::genesis_block(now);

        let mut block1: Block = Block::new(
            "PARTY_ONE".to_string(),
            "PARTY_TWO".to_string(),
            now,
            1,
            vec![0; 32],
            "".to_string(),
        );

        for x in 0..u128::MAX {
            let block = Block::new(
                "PARTY_ONE".to_string(),
                "PARTY_TWO".to_string(),
                now,
                x,
                vec![0; 32],
                genesis_block.clone().hash.unwrap(),
            );

            if block.verify_block() {
                block1 = block;
                break;
            } else {
                continue;
            }
        }

        let mut blockchain = Blockchain::start(genesis_block.clone());

        blockchain.add_block(block1.clone(), "wallet".to_string());

        let blocked = blockchain.blocks.get(1).unwrap().to_owned();

        assert!(blocked.clone().hash.unwrap().starts_with(
            repeat("0")
                .take(blocked.difficulty.unwrap().into())
                .collect::<String>()
                .as_str()
        ));
    }
}
