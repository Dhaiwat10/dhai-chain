use crate::transaction::Transaction;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MempoolError {
    #[error("Transaction already exists")]
    DuplicateTransaction,
    #[error("Invalid transaction")]
    InvalidTransaction,
}

// Wrapper for Transaction to implement Ord for the priority queue
#[derive(Debug, Clone, Eq, PartialEq)]
struct PrioritizedTransaction(Transaction);

impl Ord for PrioritizedTransaction {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison for min-heap (lower nonce = higher priority)
        other.0.nonce().cmp(&self.0.nonce())
    }
}

impl PartialOrd for PrioritizedTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Mempool {
    transactions: HashMap<[u8; 32], Transaction>,
    priority_queue: BinaryHeap<PrioritizedTransaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            priority_queue: BinaryHeap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), MempoolError> {
        let tx_hash = transaction.hash();

        if self.transactions.contains_key(&tx_hash) {
            return Err(MempoolError::DuplicateTransaction);
        }

        if let Err(_) = transaction.validate(false) {
            // Regular transactions are never genesis
            return Err(MempoolError::InvalidTransaction);
        }

        self.priority_queue
            .push(PrioritizedTransaction(transaction.clone()));
        self.transactions.insert(tx_hash, transaction);

        Ok(())
    }

    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        let all_txs = self
            .priority_queue
            .iter()
            .map(|pt| pt.0.clone())
            .collect::<Vec<_>>();

        let mut sorted = all_txs;
        sorted.sort_by_key(|tx| tx.nonce());
        sorted.into_iter().take(limit).collect()
    }

    pub fn remove_transactions(&mut self, transactions: &[Transaction]) {
        for tx in transactions {
            let tx_hash = tx.hash();
            self.transactions.remove(&tx_hash);
            // Note: This is inefficient as we're rebuilding the heap
            // In a real implementation, we might want a better data structure
            self.priority_queue = self
                .priority_queue
                .iter()
                .filter(|pt| pt.0.hash() != tx_hash)
                .cloned()
                .collect();
        }
    }

    pub fn contains(&self, transaction: &Transaction) -> bool {
        self.transactions.contains_key(&transaction.hash())
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
        self.priority_queue.clear();
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

#[cfg(test)]
mod tests;
