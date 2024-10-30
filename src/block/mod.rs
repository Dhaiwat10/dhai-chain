use crate::transaction::{Transaction, TransactionError};
use sha2::{Digest, Sha256};
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Error, Debug)]
pub enum BlockError {
    #[error("Invalid block hash")]
    InvalidHash,
    #[error("Invalid previous hash format")]
    InvalidPreviousHash,
    #[error("Proof of work validation failed")]
    InvalidProofOfWork,
    #[error("Invalid difficulty target")]
    InvalidDifficulty,
    #[error("No transactions in block")]
    EmptyTransactions,
    #[error("Transaction error: {0}")]
    TransactionError(#[from] TransactionError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    timestamp: OffsetDateTime,
    transactions: Vec<Transaction>,
    previous_hash: [u8; 32],
    hash: [u8; 32],
    nonce: u64,
    difficulty: u32, // Number of leading zeros required
}

impl Block {
    /// Creates a new block with the given data and previous hash
    pub fn new(
        transactions: Vec<Transaction>,
        previous_hash: [u8; 32],
        difficulty: u32,
    ) -> Result<Self, BlockError> {
        if transactions.is_empty() {
            return Err(BlockError::EmptyTransactions);
        }

        let timestamp = OffsetDateTime::now_utc();
        let mut block = Self {
            timestamp,
            transactions,
            previous_hash,
            hash: [0; 32],
            nonce: 0,
            difficulty,
        };
        block.hash = block.calculate_hash();
        Ok(block)
    }

    /// Calculates the hash of the block based on its contents
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Hash timestamp
        hasher.update(self.timestamp.unix_timestamp().to_be_bytes());

        // Hash all transactions
        for transaction in &self.transactions {
            hasher.update(transaction.sender().as_bytes());
            hasher.update(transaction.receiver().as_bytes());
            hasher.update(transaction.amount().to_be_bytes());
            hasher.update(transaction.nonce().to_be_bytes());
        }

        // Hash previous hash
        hasher.update(self.previous_hash);

        // Hash nonce
        hasher.update(self.nonce.to_be_bytes());

        hasher.finalize().into()
    }

    pub fn mine(&mut self) {
        let target = 0u64.wrapping_sub(1) >> self.difficulty;
        loop {
            let hash = self.calculate_hash();
            // Convert first 8 bytes of hash to u64 for easy comparison
            let hash_num = u64::from_be_bytes(hash[0..8].try_into().unwrap());
            if hash_num <= target {
                self.hash = hash;
                break;
            }
            self.nonce = self.nonce.wrapping_add(1);
        }
    }

    pub fn has_valid_proof(&self) -> bool {
        let hash_num = u64::from_be_bytes(self.hash[0..8].try_into().unwrap());
        let target = 0u64.wrapping_sub(1) >> self.difficulty;
        hash_num <= target
    }

    pub fn verify(&self, is_genesis: bool) -> Result<(), BlockError> {
        // Verify block has transactions
        if self.transactions.is_empty() {
            return Err(BlockError::EmptyTransactions);
        }

        // Verify all transactions are valid
        for transaction in &self.transactions {
            transaction.validate(is_genesis)?;
        }

        // Verify hash and proof of work
        if self.hash != self.calculate_hash() {
            return Err(BlockError::InvalidHash);
        }

        if !self.has_valid_proof() {
            return Err(BlockError::InvalidProofOfWork);
        }

        Ok(())
    }

    // Getters
    pub fn hash(&self) -> [u8; 32] {
        self.hash
    }

    pub fn previous_hash(&self) -> [u8; 32] {
        self.previous_hash
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn difficulty(&self) -> u32 {
        self.difficulty
    }

    #[cfg(test)]
    pub fn set_transactions_for_testing(&mut self, transactions: Vec<Transaction>) {
        self.transactions = transactions;
    }
}

#[cfg(test)]
mod tests;
