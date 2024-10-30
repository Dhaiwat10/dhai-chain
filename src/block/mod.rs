use sha2::{Sha256, Digest};
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    timestamp: OffsetDateTime,
    data: Vec<u8>,
    previous_hash: [u8; 32],
    hash: [u8; 32],
    nonce: u64,
    difficulty: u32,  // Number of leading zeros required
}

impl Block {
    /// Creates a new block with the given data and previous hash
    pub fn new(data: Vec<u8>, previous_hash: [u8; 32], difficulty: u32) -> Self {
        let timestamp = OffsetDateTime::now_utc();
        let mut block = Self {
            timestamp,
            data,
            previous_hash,
            hash: [0; 32],
            nonce: 0,
            difficulty,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block based on its contents
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Hash timestamp
        hasher.update(self.timestamp.unix_timestamp().to_be_bytes());
        
        // Hash data
        hasher.update(&self.data);
        
        // Hash previous hash
        hasher.update(self.previous_hash);

        // Hash nonce
        hasher.update(self.nonce.to_be_bytes());
        
        hasher.finalize().into()
    }

    pub fn mine(&mut self) {
        while !self.has_valid_proof() {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    fn has_valid_proof(&self) -> bool {
        let hash_bytes = self.hash;
        let required_zeros = (self.difficulty / 8) as usize;
        let remaining_bits = (self.difficulty % 8) as u32;

        // Check whole bytes first
        if !hash_bytes.iter().take(required_zeros).all(|&byte| byte == 0) {
            return false;
        }

        // Check remaining bits if any
        if remaining_bits > 0 {
            let next_byte = hash_bytes[required_zeros];
            let mask = 0xFF_u8 >> remaining_bits;
            if next_byte & !mask != 0 {
                return false;
            }
        }

        true
    }

    pub fn verify(&self) -> Result<(), BlockError> {
        // First verify the basic hash validity
        if self.hash != self.calculate_hash() {
            return Err(BlockError::InvalidHash);
        }

        // Then verify the proof of work
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

    pub fn data(&self) -> &[u8] {
        &self.data
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
}

#[cfg(test)]
mod tests;