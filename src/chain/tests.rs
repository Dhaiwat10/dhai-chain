use super::*;
use crate::transaction::{Transaction, Address};
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn create_test_address(value: u8) -> Address {
    Address::new([value; 20])
}

fn create_test_transaction() -> Transaction {
    let nonce = NONCE_COUNTER.fetch_add(1, Ordering::SeqCst);
    Transaction::new(
        create_test_address(1),  // sender
        create_test_address(2),  // receiver
        100,                     // amount
        nonce,                   // unique nonce for each test
    )
}

fn create_test_chain(difficulty: Option<u32>, genesis_tx: Option<Transaction>) -> Result<Chain, ChainError> {
    let difficulty = difficulty.unwrap_or(1);
    Chain::new(difficulty, genesis_tx)
}

#[test]
fn test_new_chain_creation() {
    let chain = create_test_chain(None, None).unwrap();
    
    assert_eq!(chain.len(), 1); // Should have genesis block
    assert_eq!(chain.current_difficulty(), 1);
    
    // Verify genesis block
    let genesis = chain.get_block(0).unwrap();
    assert_eq!(genesis.previous_hash(), [0; 32]);
    assert!(!chain.is_empty());
}

#[test]
fn test_add_block_with_mempool() {
    let mut chain = create_test_chain(None, None).unwrap();
    
    // Submit transactions to mempool
    chain.submit_transaction(create_test_transaction()).unwrap();
    chain.submit_transaction(create_test_transaction()).unwrap();
    
    // Add block should use mempool transactions
    chain.add_block().unwrap();
    
    assert_eq!(chain.len(), 2);
    let block = chain.get_block(1).unwrap();
    assert!(!block.transactions().is_empty());
    
    // Mempool should be empty after block creation
    chain.add_block().unwrap();
    assert_eq!(chain.len(), 2); // Shouldn't add empty block
}

#[test]
fn test_block_with_specific_transactions() {
    let mut chain = create_test_chain(None, None).unwrap();
    let transactions = vec![create_test_transaction(), create_test_transaction()];
    
    chain.add_block_with_transactions(transactions.clone()).unwrap();
    
    assert_eq!(chain.len(), 2);
    let block = chain.get_block(1).unwrap();
    assert_eq!(block.transactions(), &transactions);
}

#[test]
fn test_mempool_ordering() {
    let mut chain = create_test_chain(None, None).unwrap();
    
    // Add transactions in reverse order
    let tx3 = create_test_transaction(); // nonce 2
    let tx2 = create_test_transaction(); // nonce 1
    let tx1 = create_test_transaction(); // nonce 0
    
    chain.submit_transaction(tx3).unwrap();
    chain.submit_transaction(tx2).unwrap();
    chain.submit_transaction(tx1).unwrap();
    
    chain.add_block().unwrap();
    
    let block = chain.get_block(1).unwrap();
    let block_txs = block.transactions();
    assert_eq!(block_txs.len(), 3);
    assert!(block_txs[0].nonce() < block_txs[1].nonce());
    assert!(block_txs[1].nonce() < block_txs[2].nonce());
}

#[test]
fn test_chain_verification() {
    let mut chain = create_test_chain(None, None).unwrap();
    
    // Add some transactions and create blocks
    chain.submit_transaction(create_test_transaction()).unwrap();
    chain.add_block().unwrap();
    
    chain.submit_transaction(create_test_transaction()).unwrap();
    chain.add_block().unwrap();

    assert!(chain.verify().is_ok());
}

#[test]
fn test_empty_chain_verification() {
    let mut chain = create_test_chain(None, None).unwrap();
    chain.blocks.clear(); // Force empty chain for testing
    
    assert!(matches!(chain.verify(), Err(ChainError::EmptyChain)));
}

#[test]
fn test_block_difficulty_matches_chain() {
    let difficulty = 2;
    let mut chain = Chain::new(difficulty, None).unwrap();
    
    chain.submit_transaction(create_test_transaction()).unwrap();
    chain.add_block().unwrap();
    
    let block = chain.latest_block().unwrap();
    assert_eq!(block.difficulty(), difficulty);
}

#[test]
fn test_chain_tamper_detection() {
    let mut chain = create_test_chain(None, None).unwrap();
    
    // Add a valid block first
    chain.submit_transaction(create_test_transaction()).unwrap();
    chain.add_block().unwrap();
    
    // Tamper with the last block
    if let Some(block) = chain.blocks.last_mut() {
        block.set_transactions_for_testing(vec![create_test_transaction()]);
    }

    assert!(chain.verify().is_err());
}

#[test]
fn test_invalid_genesis_detection() {
    // Create two chains with different genesis transactions
    let tx1 = create_test_transaction();
    let tx2 = create_test_transaction();
    
    let chain1 = create_test_chain(None, Some(tx1)).unwrap();
    let chain2 = create_test_chain(None, Some(tx2)).unwrap();
    
    assert_ne!(
        chain1.get_block(0).unwrap().hash(),
        chain2.get_block(0).unwrap().hash()
    );
}