use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64
}