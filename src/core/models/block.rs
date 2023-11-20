use crate::core::{
    hashing::{calculate_hash, hash_to_binary_representation},
    mining::DIFFICULTY_PREFIX,
};
use chrono::Utc;
use hex::FromHexError;

use super::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub txs: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn genesis() -> Block {
        Block {
            height: 0,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
            timestamp: Utc::now().timestamp(),
            txs: Vec::new(),
            nonce: 2836
        }
    }

    pub fn new(prev_block: &Block, hash: String, timestamp: i64, txs: Vec<Transaction>, nonce: u64) -> Block {
        Block {
            height: prev_block.height + 1,
            hash: hash,
            previous_hash: prev_block.hash.clone(),
            timestamp: timestamp,
            txs: txs,
            nonce: nonce
        }
    }

    pub fn verify(&self, prev_block: &Block) -> Result<bool, FromHexError> {
        if self.previous_hash != prev_block.hash {
            return Ok(false);
        }

        if self.height != prev_block.height + 1 {
            return Ok(false);
        }

        let decoded_hash = &hex::decode(&self.hash)?;
        if !hash_to_binary_representation(&decoded_hash).starts_with(DIFFICULTY_PREFIX) {
            return Ok(false);
        }

        let encoded_hash = hex::encode(calculate_hash(
            self.height,
            self.timestamp,
            &self.previous_hash,
            &self.txs,
            self.nonce,
        ));

        if encoded_hash != self.hash {
            return Ok(false);
        }

        Ok(true)
    }
}
