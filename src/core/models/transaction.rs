use serde::Serialize;

#[derive(Debug, Serialize, Clone, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
}

// impl Ord for Transaction {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         other
//             .fee
//             .cmp(&self.fee)
//             .then_with(|| other.amount.cmp(&self.amount))
//     }
// }

// impl PartialOrd for Transaction {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}

#[derive(Debug, Serialize, Clone, Eq)]
pub struct TransactionPriority {
    pub nonce: u64,
    pub fee: u64,
    pub amount: u64,
}

impl TransactionPriority {
    pub fn new(nonce: u64, fee: u64, amount: u64) -> TransactionPriority {
        TransactionPriority {
            nonce: nonce,
            fee: fee,
            amount: amount,
        }
    }

    pub fn new_from_tx(tx: &Transaction) -> TransactionPriority {
        TransactionPriority {
            nonce: tx.nonce,
            fee: tx.fee,
            amount: tx.amount
        }
    }
}

impl Ord for TransactionPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .fee
            .cmp(&self.fee)
            .then_with(|| other.amount.cmp(&self.amount))
    }
}

impl PartialOrd for TransactionPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TransactionPriority {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}
