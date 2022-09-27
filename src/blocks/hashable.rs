use crate::utils::{hashing, json};

use super::Block;

pub trait Hashable {
    fn gen_hash(&self) -> String;
}

impl Hashable for Block {
    fn gen_hash(&self) -> String {
        hashing::hash_string(json::ser_json(self))
    }
}
