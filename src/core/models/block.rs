use crate::core::{
    hashing::{calculate_hash, hash_to_binary_representation},
    mining::DIFFICULTY_PREFIX,
};
use hex::FromHexError;

pub struct Block {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    fn verify(&self, prev_block: &Block) -> Result<bool, FromHexError> {
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
            &self.data,
            self.nonce,
        ));

        if encoded_hash != self.hash {
            return Ok(false);
        }

        Ok(true)
    }
}
