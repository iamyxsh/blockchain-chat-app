use crate::{
    blockchain::Blockchain,
    utils::{
        db::{CRUD, KVDB},
        json::ser_json,
    },
};

pub trait Savable {
    fn save(&self);
    fn load(&self) -> String;
}

impl Savable for Blockchain {
    fn save(&self) {
        let db = KVDB::init("tmp");

        db.save("blockchain", &ser_json(self));
        let _ = db.db.flush();
    }

    fn load(&self) -> String {
        let db = KVDB::init("tmp");

        let blockchain = db.find("blockchain").unwrap();
        let _ = db.db.flush();

        blockchain
    }
}
