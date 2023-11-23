use std::collections::{BTreeSet, HashMap};

use super::{Transaction, TransactionPriority, TransactionValidationError};

pub struct MemPool {
    prioritized_txs: BTreeSet<TransactionPriority>,
    txs: HashMap<String, Transaction>,
    max_cap: usize,
}

impl MemPool {
    pub fn new(max_cap: usize) -> MemPool {
        MemPool {
            prioritized_txs: BTreeSet::new(),
            txs: HashMap::new(),
            max_cap,
        }
    }

    pub fn add_tx(&mut self, tx: Transaction) -> Result<(), TransactionValidationError> {
        tx.validate()?;

        if self.txs.len() == self.max_cap {
            self.evict_tx();
        }

        let tx_priority = TransactionPriority::new_from_tx(&tx);
        if let Some(already_existing_tx) = self.txs.insert(tx.nonce.clone(), tx) {
            self.prioritized_txs
                .retain(|t| t.nonce != already_existing_tx.nonce);
        }
        self.prioritized_txs.insert(tx_priority);

        assert_eq!(self.prioritized_txs.len(), self.txs.len());

        Ok(())
    }

    pub fn take_txs_w_limit(&mut self, limit: usize) -> Vec<Transaction> {
        let mut result: Vec<Transaction> = Vec::new();
        for _ in 0..limit {
            let tx_priority_opt = self.prioritized_txs.pop_first();
            if let Some(tx_priority) = tx_priority_opt {
                if let Some(tx) = self.txs.remove(&tx_priority.nonce) {
                    result.push(tx);
                }
            } else {
                break;
            }
        }
        assert_eq!(self.prioritized_txs.len(), self.txs.len());

        result
    }

    pub fn get_tx(&self, nonce: &str) -> Option<&Transaction> {
        self.txs.get(nonce)
    }

    pub fn evict_tx(&mut self) -> Option<Transaction> {
        if let Some(ev_tx) = self.prioritized_txs.pop_last() {
            let evicted_nonce = ev_tx.nonce;
            return self.txs.remove(&evicted_nonce);
        }

        None
    }

    pub fn remove_tx(&mut self, nonce: &str) -> Option<Transaction> {
        if let Some(removed_tx) = self.txs.remove(nonce) {
            let tx_prior_to_remove = TransactionPriority::new_from_tx(&removed_tx);
            self.prioritized_txs.remove(&tx_prior_to_remove);
            return Some(removed_tx);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }
}

#[cfg(test)]
mod memory_pool_test {
    use std::ptr::null;

    use crate::core::Transaction;

    use super::MemPool;

    #[test]
    fn add_tx_to_mempool_with_space_adds_the_tx() {
        let mut mempool = MemPool::new(10);

        assert_eq!(0, mempool.len());

        let add_res = mempool.add_tx(Transaction::new(
            "from_address".to_string(),
            "to_string".to_string(),
            1234500,
            100,
        ));

        assert!(add_res.is_ok());
        assert_eq!(1, mempool.len());
    }

    #[test]
    fn adding_new_tx_when_max_capacity_removes_tx_with_lower_fee_in_place_of_the_new_one() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        let mut tx_low_fee_nonce: String = String::from("");
        for i in 1..=5 {
            let tx_to_add = Transaction::new(
                "from_address".to_string(),
                "to_string".to_string(),
                1234500,
                15 - i,
            );
            tx_low_fee_nonce = tx_to_add.nonce.clone();
            let add_res = mempool.add_tx(tx_to_add);

            assert!(add_res.is_ok());
        }

        assert_eq!(5, mempool.len());

        let new_tx = Transaction::new(
            "from_address".to_string(),
            "to_string".to_string(),
            1234500,
            5,
        );
        let new_tx_nonce = new_tx.nonce.clone();
        let add_res = mempool.add_tx(new_tx);

        assert!(add_res.is_ok());
        assert_eq!(5, mempool.len());
        assert!(mempool.get_tx(&tx_low_fee_nonce).is_none());
        assert!(mempool.get_tx(&new_tx_nonce).is_some());
    }

    #[test]
    fn take_n_txs_returns_n_txs_if_enough_txs() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());
        let mut inserted_nonce = Vec::new();

        for i in 0..5 {
            let tx_to_add = Transaction::new(
                "from_address".to_string(),
                "to_string".to_string(),
                1234500,
                i + 10,
            );
            inserted_nonce.push(tx_to_add.nonce.clone());
            let add_res = mempool.add_tx(tx_to_add);

            assert!(add_res.is_ok());
        }

        assert_eq!(5, mempool.len());

        let retrieved_txs = mempool.take_txs_w_limit(3);

        assert_eq!(3, retrieved_txs.len());
        assert_eq!(inserted_nonce[4], retrieved_txs[0].nonce);
        assert_eq!(inserted_nonce[3], retrieved_txs[1].nonce);
        assert_eq!(inserted_nonce[2], retrieved_txs[2].nonce);

        let left_txs = mempool.take_txs_w_limit(10);
        assert_eq!(2, left_txs.len());
        assert_eq!(inserted_nonce[1], left_txs[0].nonce);
        assert_eq!(inserted_nonce[0], left_txs[1].nonce);

        let empty_txs = mempool.take_txs_w_limit(10);
        assert_eq!(0, empty_txs.len());
    }

    #[test]
    fn remove_tx_returns_the_removed_tx_if_any() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        let tx = Transaction::new(
            "from_address".to_string(),
            "to_string".to_string(),
            1234500,
            100,
        );
        let tx_nonce = tx.nonce.to_string();
        let add_res = mempool.add_tx(tx);

        assert!(add_res.is_ok());
        assert_eq!(1, mempool.len());

        let removed_tx_opt = mempool.remove_tx(&tx_nonce);
        assert!(removed_tx_opt.is_some_and(|t| t.nonce == tx_nonce));

        let non_existing_tx = mempool.remove_tx(&tx_nonce);
        assert!(non_existing_tx.is_none());
    }

    #[test]
    fn adding_two_txs_with_same_nonce_will_replace_the_tx() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        let tx = Transaction::new(
            "from_address".to_string(),
            "to_string".to_string(),
            1234500,
            100,
        );
        let tx_nonce = tx.nonce.clone();
        let add_res = mempool.add_tx(tx);

        assert!(add_res.is_ok());
        assert_eq!(1, mempool.len());

        let tx2 = Transaction::new(
            "from_address".to_string(),
            "to_string".to_string(),
            1234500,
            100,
        );
        let add_res2 = mempool.add_tx(tx2);

        assert!(add_res2.is_ok());
        assert_eq!(1, mempool.len());

        let tx = mempool.get_tx(&tx_nonce);
        assert!(
            tx.is_some_and(|t| t.nonce == tx_nonce && t.amount == 1234500 && t.fee == 100)
        );
    }
}
