#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use sha256::digest;

    use crate::utils::{hashing::hash_string, json::ser_json};

    #[derive(Serialize, Deserialize, Debug)]
    struct SampleStruct {
        name: String,
    }

    #[test]
    fn it_should_hash_string() {
        let sample_string = "Sample String".to_string();
        let hashed_string = hash_string(sample_string.clone());

        assert_eq!(hashed_string, digest(sample_string.clone()));
    }

    #[test]
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
}
