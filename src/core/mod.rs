mod models;
mod mining;
mod hashing;
mod history;
mod errors;
mod memory_pool;

pub type Block = models::block::Block;
pub type Transaction = models::transaction::Transaction;
pub type History = history::History;

pub type NaiveReorgStrategy = history::NaiveReorgStrategy;

pub type AppendToHistoryError = errors::AppendToHistoryError;

pub use mining::mine_new_block as mine_new_block;