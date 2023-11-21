use serde::Serialize;

#[derive(Debug, Serialize, Clone, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.fee.cmp(&self.fee)
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}