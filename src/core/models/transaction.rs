use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64
}