use sled::{open, Db, IVec};

pub trait CRUD {
    fn init() -> Self;
    fn save(&self, k: &str, v: &str);
    fn find(&self, k: &str) -> Option<IVec>;
}

#[derive(Clone)]
pub struct KVDB {
    pub db: sled::Db,
}

impl CRUD for KVDB {
    fn init() -> Self {
        let db: Db = open("tmp").unwrap();
        KVDB { db }
    }

    fn save(&self, k: &str, v: &str) {
        let _ = self.db.insert(k.as_bytes(), v.as_bytes());
    }

    fn find(&self, k: &str) -> Option<IVec> {
        self.db.get(k.as_bytes()).unwrap()
    }
}
