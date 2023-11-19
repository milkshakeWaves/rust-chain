use hex;
use sha2::{Digest, Sha256};

pub fn mine_new_block(
    height: u64,
    timestamp: i64,
    previous_hash: &str,
    data: &str,
) -> (u64, String) {
    println!("Mining new block...");

    let mut nonce: u64 = 0;

    loop {
        let hash = calculate_hash(height, timestamp, previous_hash, data, nonce);
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

fn calculate_hash(
    height: u64,
    timestamp: i64,
    previous_hash: &str,
    data: &str,
    nonce: u64,
) -> Vec<u8> {
    let data = serde_json::json!({
        "height": height,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

const DIFFICULTY_PREFIX: &str = "00";
