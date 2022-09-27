#[cfg(test)]
mod tests {
    use crate::{
        blocks::Block,
        utils::{hashing::hash_string, json::ser_json},
    };
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn it_should_init_genesis_block() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let block = Block::new(
            "PARTY ONE".to_string(),
            "PARTY TWO".to_string(),
            now,
            100,
            vec![0; 32],
        );

        let hash = hash_string(ser_json(&Block {
            timestamp: now,
            difficulty: None,
            hash: None,
            previous_hash: None,
            party_one: "PARTY ONE".to_string(),
            party_two: "PARTY TWO".to_string(),
            nonce: 100,
            payload: vec![0; 32],
        }));

        assert_eq!(block.party_one, "PARTY ONE".to_string());
        assert_eq!(block.party_two, "PARTY TWO".to_string());
        assert_eq!(block.timestamp, now);
        assert_eq!(block.nonce, 100);
        assert_eq!(block.payload, vec![0; 32]);
        assert_eq!(block.hash.unwrap(), hash);
        assert_eq!(block.previous_hash, None);
    }
}
