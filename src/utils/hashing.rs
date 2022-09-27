use sha256::digest;

pub fn hash_string(info: String) -> String {
    print!("info: {}", info);
    digest(info)
}
