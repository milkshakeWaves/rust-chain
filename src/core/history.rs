use super::{Block, AppendToHistoryError};

pub struct History {
    pub chain: Vec<Block>
}

impl History {
    pub fn new() -> History {
        History { chain: vec![Block::genesis()] }
    }

    pub fn try_to_append(&mut self, new_block: Block) -> Result<bool, AppendToHistoryError> {
        let tail_block = self.chain.last().ok_or(AppendToHistoryError{})?;

        new_block.verify(tail_block)?;

        self.chain.push(new_block);

        Ok(true)
    }

    pub fn get_height(&self) -> usize {
        self.chain.len()
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }
}