use serde::Serialize;

use crate::core::{hashing::calculate_tx_nonce, TransactionValidationError};

#[derive(Debug, Serialize, Clone, Eq)]
pub struct Transaction {
    pub nonce: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Transaction {
        let nonce = calculate_tx_nonce(&from, &to, amount, fee);
        Transaction {
            nonce: nonce,
            from: from,
            to: to,
            amount: amount,
            fee: fee,
        }
    }

    pub fn validate(&self) -> Result<(), TransactionValidationError> {
        let correct_nonce = calculate_tx_nonce(&self.from, &self.to, self.amount, self.fee);
        if correct_nonce == self.nonce {
            Ok(())
        } else {
            Err(TransactionValidationError {})
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}

#[derive(Debug, Serialize, Clone, Eq)]
pub struct TransactionPriority {
    pub nonce: String,
    pub fee: u64,
    pub amount: u64,
}

impl TransactionPriority {
    pub fn new(nonce: String, fee: u64, amount: u64) -> TransactionPriority {
        TransactionPriority {
            nonce: nonce,
            fee: fee,
            amount: amount,
        }
    }

    pub fn new_from_tx(tx: &Transaction) -> TransactionPriority {
        TransactionPriority {
            nonce: tx.nonce.to_string(),
            fee: tx.fee,
            amount: tx.amount,
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

#[cfg(test)]
mod transaction_test {
    use crate::core::Transaction;

    #[test]
    fn verify_correct_nonce_returns_true() {
        let tx = Transaction::new(
            "from-address".to_string(),
            "to-address".to_string(),
            12345,
            100,
        );

        assert!(tx.validate().is_ok());
    }

    #[test]
    fn verify_bad_nonce_returns_false() {
        let mut tx = Transaction::new(
            "from-address".to_string(),
            "to-address".to_string(),
            12345,
            100,
        );

        tx.nonce = "bad-nonce".to_string();

        assert!(tx.validate().is_err());

        let tx2 = Transaction{
            nonce: "another-bad-nonce".to_string(),
            from: "from-address".to_string(),
            to: "to-address".to_string(),
            amount: 12345,
            fee: 100,
        };

        assert!(tx2.validate().is_err());
    }
}
