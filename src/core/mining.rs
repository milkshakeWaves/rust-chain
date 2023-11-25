use hex;
use crate::core::hashing::{hash_to_binary_representation, calculate_hash};
use super::Transaction;

pub fn mine_new_block(
    height: u64,
    timestamp: i64,
    previous_hash: &str,
    txs: &Vec<Transaction>,
) -> (u64, String) {
    println!("Mining new block...");

    let mut nonce: u64 = 0;

    loop {
        if nonce % 100000 == 0 {
            println!("Still computing...");
        }

        let data = serde_json::json!({
            "height": height,
            "previous_hash": previous_hash,
            "txs": txs,
            "timestamp": timestamp,
            "nonce": nonce
        });
        let hash = calculate_hash(&data);
        let binary_hash = hash_to_binary_representation(&hash);

        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            println!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }

        nonce += 1;
    }
}

pub const DIFFICULTY_PREFIX: &str = "00";