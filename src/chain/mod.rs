use crate::block::{Block, BlockError};
use crate::mempool::{Mempool, MempoolError};
use crate::transaction::{Address, Transaction};
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
    #[error("Mempool error: {0}")]
    MempoolError(#[from] MempoolError),
}

pub struct Chain {
    blocks: Vec<Block>,
    current_difficulty: u32,
    mempool: Mempool,
}

impl Chain {
    pub fn new(difficulty: u32, genesis_tx: Option<Transaction>) -> Result<Self, ChainError> {
        let genesis_tx = genesis_tx.unwrap_or_else(|| {
            Transaction::new(
                Address::new([0; 20]), // Genesis sender
                Address::new([0; 20]), // Same address for genesis
                1,                     // Genesis amount
                0,                     // Genesis nonce
            )
        });

        let mut genesis_block = Block::new(vec![genesis_tx], [0; 32], difficulty)?;

        genesis_block.mine();

        Ok(Self {
            blocks: vec![genesis_block],
            current_difficulty: difficulty,
            mempool: Mempool::new(),
        })
    }

    pub fn submit_transaction(&mut self, transaction: Transaction) -> Result<(), ChainError> {
        self.mempool.add_transaction(transaction)?;
        Ok(())
    }

    pub fn add_block(&mut self) -> Result<(), ChainError> {
        let previous_block = self.blocks.last().ok_or(ChainError::EmptyChain)?;

        let transactions = self.mempool.get_transactions(10);

        if transactions.is_empty() {
            // todo: handle empty mempool
            return Ok(());
        }

        let mut new_block = Block::new(
            transactions.clone(),
            previous_block.hash(),
            self.current_difficulty,
        )?;

        new_block.mine();
        new_block.verify(false)?;

        self.mempool.remove_transactions(&transactions);
        self.blocks.push(new_block);

        Ok(())
    }

    pub fn add_block_with_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), ChainError> {
        let previous_block = self.blocks.last().ok_or(ChainError::EmptyChain)?;

        let mut new_block =
            Block::new(transactions, previous_block.hash(), self.current_difficulty)?;

        new_block.mine();
        new_block.verify(false)?;

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
        if genesis_block.previous_hash() != [0; 32] {
            return Err(ChainError::InvalidGenesis);
        }
        genesis_block.verify(true)?;

        // verify rest of the chain
        for window in self.blocks.windows(2) {
            let previous_block = &window[0];
            let current_block = &window[1];

            if current_block.previous_hash() != previous_block.hash() {
                return Err(ChainError::InvalidBlockLink);
            }

            current_block.verify(false)?;
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
