mod models;
mod mining;
mod hashing;
mod history;
mod errors;
mod memory_pool;
mod wallet;

pub type Block = models::block::Block;
pub type Transaction = models::transaction::Transaction;
pub type TransactionPriority = models::transaction::TransactionPriority;
pub type History = history::History;
pub type WalletKeyPair = wallet::WalletKeyPair;

pub type NaiveReorgStrategy = history::NaiveReorgStrategy;

pub type AppendToHistoryError = errors::AppendToHistoryError;
pub type TransactionValidationError = errors::TransactionValidationError;
pub type EmptySignatureError = errors::EmptySignatureError;

pub use mining::mine_new_block as mine_new_block;