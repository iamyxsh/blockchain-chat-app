use sha256::digest;

pub fn hash_string(info: String) -> String {
    digest(info)
}
