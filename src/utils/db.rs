use rocksdb::DB;
use std::sync::Arc;
pub trait CRUD {
    fn init(file_path: &str) -> Self;
    fn save(&self, k: &str, v: &str) -> bool;
    fn find(&self, k: &str) -> Option<String>;
    fn delete(&self, k: &str) -> bool;
}
#[derive(Clone)]
pub struct KVDB {
    pub db: Arc<DB>,
}
impl CRUD for KVDB {
    fn init(file_path: &str) -> Self {
        KVDB {
            db: Arc::new(DB::open_default(file_path).unwrap()),
        }
    }
    fn save(&self, k: &str, v: &str) -> bool {
        self.db.put(k.as_bytes(), v.as_bytes()).is_ok()
    }
    fn find(&self, k: &str) -> Option<String> {
        match self.db.get(k.as_bytes()) {
            Ok(Some(v)) => {
                let result = String::from_utf8(v).unwrap();

                Some(result)
            }
            Ok(None) => None,
            Err(_) => None,
        }
    }
    fn delete(&self, k: &str) -> bool {
        self.db.delete(k.as_bytes()).is_ok()
    }
}
