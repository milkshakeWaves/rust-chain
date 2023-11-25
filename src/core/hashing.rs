use serde_json::Value;
use sha2::{Digest, Sha256};

pub fn calculate_hash(data: &Value) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    let hash = hasher.finalize();
    Sha256::digest(hash).as_slice().try_into().expect("")
}

pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

#[cfg(test)]
mod hashing_test {
    use super::calculate_hash;

    #[test]
    fn create_32_len_hash() {
        let data = serde_json::json!({
            "height": "height",
            "previous_hash": "previous_hash",
            "txs": Vec::<String>::new(),
            "timestamp": 123456789,
            "nonce": "nonce"
        });

        let hash = calculate_hash(&data);

        assert_eq!(32, hash.len());
    }
}