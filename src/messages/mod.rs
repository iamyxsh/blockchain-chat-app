mod __tests__;
mod hashable;

use serde::{Deserialize, Serialize};

use crate::blocks::hashable::Hashable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub message: String,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
}

impl Message {
    pub fn create(from: String, to: String, message: String, previous_hash: String) -> Self {
        let mut msg = Message {
            from,
            to,
            message,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash,
            hash: "".to_string(),
        };

        let hash = msg.gen_hash();
        msg.hash = hash;

        msg
    }

    pub fn genesis(from: String, to: String, message: String) -> Self {
        let mut msg = Message {
            from,
            to,
            message,
            timestamp: chrono::Utc::now().timestamp(),
            previous_hash: "".to_string(),
            hash: "".to_string(),
        };

        let hash = msg.gen_hash();
        msg.hash = hash;

        msg
    }
}
