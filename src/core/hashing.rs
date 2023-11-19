use sha2::{Digest, Sha256};
use super::Transaction;

pub fn calculate_hash(
    height: u64,
    timestamp: i64,
    previous_hash: &str,
    txs: &Vec<Transaction>,
    nonce: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
        "height": height,
        "previous_hash": previous_hash,
        "txs": txs,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
