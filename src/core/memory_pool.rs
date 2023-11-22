use std::collections::{BTreeSet, HashMap};

use super::{Transaction, TransactionPriority};

pub struct MemPool {
    prioritized_txs: BTreeSet<TransactionPriority>,
    txs: HashMap<u64, Transaction>,
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

    pub fn add_tx(&mut self, tx: Transaction) -> () {
        // todo: validate tx
        if self.txs.len() == self.max_cap {
            self.evict_tx();
        }
        
        let tx_priority = TransactionPriority::new_from_tx(&tx);
        if let Some(already_existing_tx) = self.txs.insert(tx.nonce, tx) {
            self.prioritized_txs.retain(|t| t.nonce != already_existing_tx.nonce);
        }
        self.prioritized_txs.insert(tx_priority);
        
        assert_eq!(self.prioritized_txs.len(), self.txs.len());
    }

    pub fn take_txs_w_limit(&mut self, limit: usize) -> Vec<Transaction> {
        let mut result: Vec<Transaction> = Vec::new();
        for _ in 0..limit {
            let tx_priority_opt = self.prioritized_txs.pop_first();
            if let Some(tx_priority) = tx_priority_opt {
                if let Some(tx) = self.txs.remove(&tx_priority.nonce) {
                    result.push(tx);
                }
            }
        }
        assert_eq!(self.prioritized_txs.len(), self.txs.len());

        result
    }

    pub fn get_tx(&self, nonce: u64) -> Option<&Transaction> {
        self.txs.get(&nonce)
    }

    pub fn remove_tx(&mut self, nonce: u64) -> Option<Transaction> {
        self.txs.remove(&nonce)
    }

    pub fn evict_tx(&mut self) -> Option<Transaction> {
        if let Some(ev_tx) = self.prioritized_txs.pop_last() {
            let evicted_nonce = ev_tx.nonce;
            return self.remove_tx(evicted_nonce);
        }

        None
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }
}

#[cfg(test)]
mod memory_pool_test {
    use crate::core::Transaction;

    use super::MemPool;

    #[test]
    fn add_tx_to_mempool_with_space_adds_the_tx() {
        let mut mempool = MemPool::new(10);

        assert_eq!(0, mempool.len());

        mempool.add_tx(Transaction {
            nonce: 123456,
            from: "from_address".to_string(),
            to: "to_string".to_string(),
            amount: 1234500,
            fee: 100,
        });

        assert_eq!(1, mempool.len());
    }

    #[test]
    fn adding_new_tx_when_max_capacity_removes_tx_with_lower_fee_in_place_of_the_new_one() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        for i in 1..=5 {
            mempool.add_tx(Transaction {
                nonce: i + 100,
                from: "from_address".to_string(),
                to: "to_string".to_string(),
                amount: 1234500,
                fee: 15 - i,
            });
        }

        assert_eq!(5, mempool.len());

        mempool.add_tx(Transaction {
            nonce: 123456,
            from: "from_address".to_string(),
            to: "to_string".to_string(),
            amount: 1234500,
            fee: 5,
        });

        assert_eq!(5, mempool.len());
        assert!(mempool.get_tx(105).is_none());
        assert!(mempool.get_tx(123456).is_some());
    }

    #[test]
    fn take_n_txs_returns_n_txs_if_enough_txs() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        for i in 0..5 {
            mempool.add_tx(Transaction {
                nonce: i + 100,
                from: "from_address".to_string(),
                to: "to_string".to_string(),
                amount: 1234500,
                fee: i + 10,
            });
        }

        assert_eq!(5, mempool.len());

        let retrieved_txs = mempool.take_txs_w_limit(3);

        assert_eq!(3, retrieved_txs.len());
        assert_eq!(104, retrieved_txs[0].nonce);
        assert_eq!(103, retrieved_txs[1].nonce);
        assert_eq!(102, retrieved_txs[2].nonce);

        let left_txs = mempool.take_txs_w_limit(10);
        assert_eq!(2, left_txs.len());
        assert_eq!(101, left_txs[0].nonce);
        assert_eq!(100, left_txs[1].nonce);

        let empty_txs = mempool.take_txs_w_limit(10);
        assert_eq!(0, empty_txs.len());
    }

    #[test]
    fn remove_tx_returns_the_removed_tx_if_any() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        mempool.add_tx(Transaction {
            nonce: 123456,
            from: "from_address".to_string(),
            to: "to_string".to_string(),
            amount: 1234500,
            fee: 100,
        });

        assert_eq!(1, mempool.len());

        let removed_tx_opt = mempool.remove_tx(123456);
        assert!(removed_tx_opt.is_some_and(|t| t.nonce == 123456));

        let non_existing_tx = mempool.remove_tx(123456);
        assert!(non_existing_tx.is_none());
    }

    #[test]
    fn adding_two_txs_with_same_nonce_will_replace_the_tx() {
        let mut mempool = MemPool::new(5);

        assert_eq!(0, mempool.len());

        mempool.add_tx(Transaction {
            nonce: 123456,
            from: "from_address".to_string(),
            to: "to_string".to_string(),
            amount: 1234500,
            fee: 100,
        });

        assert_eq!(1, mempool.len());

        mempool.add_tx(Transaction {
            nonce: 123456,
            from: "from_address2".to_string(),
            to: "to_string2".to_string(),
            amount: 1234500,
            fee: 80,
        });

        assert_eq!(1, mempool.len());

        let tx = mempool.get_tx(123456);
        assert!(tx.is_some_and(|t| t.nonce == 123456));
    }
}
