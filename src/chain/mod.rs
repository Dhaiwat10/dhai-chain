use crate::block::{Block, BlockError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("Invalid genesis block")]
    InvalidGenesis,
    #[error("Invalid block link")]
    InvalidBlockLink,
    #[error("Block validation failed: {0}")]
    BlockValidation(#[from] BlockError),
    #[error("Chain is empty")]
    EmptyChain,
}

pub struct Chain {
    blocks: Vec<Block>,
    current_difficulty: u32,
}

impl Chain {
    pub fn new(difficulty: u32, genesis_data: Option<Vec<u8>>) -> Self {
        let genesis_data = genesis_data.unwrap_or(b"Genesis block".to_vec());
        let genesis_block = Block::new(genesis_data, [0; 32], difficulty);

        Self {
            blocks: vec![genesis_block],
            current_difficulty: difficulty,
        }
    }

    pub fn add_block(&mut self, data: Vec<u8>) -> Result<(), ChainError> {
        let previous_block = self.blocks.last().ok_or(ChainError::EmptyChain)?;

        let mut new_block = Block::new(data, previous_block.hash(), self.current_difficulty);

        new_block.mine();

        new_block.verify()?;

        self.blocks.push(new_block);
        Ok(())
    }

    pub fn verify(&self) -> Result<(), ChainError> {
        // chain should never be empty
        if self.blocks.is_empty() {
            return Err(ChainError::EmptyChain);
        }

        // verify genesis block
        let genesis_block = &self.blocks[0];
        genesis_block.verify()?;

        // verify rest of the chain
        for window in self.blocks.windows(2) {
            let previous_block = &window[0];
            let current_block = &window[1];

            if current_block.previous_hash() != previous_block.hash() {
                return Err(ChainError::InvalidBlockLink);
            }

            current_block.verify()?;
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn current_difficulty(&self) -> u32 {
        self.current_difficulty
    }

    pub fn latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

#[cfg(test)]
mod tests;