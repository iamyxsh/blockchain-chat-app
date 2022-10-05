use crate::{
    blocks::hashable::Hashable,
    utils::{hashing, json},
};

use super::Message;

impl Hashable for Message {
    fn gen_hash(&self) -> String {
        hashing::hash_string(json::ser_json(self))
    }
}
