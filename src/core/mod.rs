mod models;
mod mining;
mod hashing;
mod history;
mod errors;

pub type Block = models::block::Block;
pub type History = history::History;

pub type AppendToHistoryError = errors::AppendToHistoryError;

pub use mining::mine_new_block as mine_new_block;