#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};
    use serial_test::serial;
    use sha256::digest;

    use crate::utils::{
        db::{CRUD, KVDB},
        hashing::hash_string,
        json::ser_json,
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct SampleStruct {
        name: String,
    }

    #[test]
    #[serial]
    fn it_should_hash_string() {
        let sample_string = "Sample String".to_string();
        let hashed_string = hash_string(sample_string.clone());

        assert_eq!(hashed_string, digest(sample_string.clone()));
    }

    #[test]
    #[serial]
    fn it_stringingify_struct() {
        let sample_struct = SampleStruct {
            name: "Yash".to_string(),
        };

        let stringified_struct = ser_json(&sample_struct);

        assert_eq!(
            stringified_struct,
            serde_json::to_string(&sample_struct).unwrap_or_default()
        );
    }

    #[test]
    #[serial]
    fn it_can_load_db_from_disk() {
        let db = KVDB::init("tmp");

        db.save("sample", &"sample".to_string());
        let find = db.find("sample".as_ref()).unwrap();

        let _ = db.db.flush().unwrap();

        assert_eq!(find, "sample");
    }
}
