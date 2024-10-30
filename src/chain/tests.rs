use super::*;
use crate::transaction::{Address, Transaction};
use std::sync::atomic::{AtomicU64, Ordering};

fn create_test_address(value: u8) -> Address {
    Address::new([value; 20])
}

// Use atomic counter for generating unique nonces
static NONCE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn create_test_transaction() -> Transaction {
    let nonce = NONCE_COUNTER.fetch_add(1, Ordering::SeqCst);
    Transaction::new(create_test_address(1), create_test_address(2), 100, nonce)
}

fn create_test_chain(
    difficulty: Option<u32>,
    genesis_tx: Option<Transaction>,
) -> Result<Chain, ChainError> {
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
fn test_adding_block() {
    let mut chain = create_test_chain(None, None).unwrap();
    let transactions = vec![create_test_transaction()];

    chain.add_block(transactions.clone()).unwrap();

    assert_eq!(chain.len(), 2);
    let block = chain.get_block(1).unwrap();
    assert_eq!(block.transactions(), &transactions);

    // Verify block links
    let genesis = chain.get_block(0).unwrap();
    assert_eq!(block.previous_hash(), genesis.hash());
}

#[test]
fn test_chain_verification() {
    let mut chain = create_test_chain(None, None).unwrap();

    // Add a few blocks
    chain.add_block(vec![create_test_transaction()]).unwrap();
    chain.add_block(vec![create_test_transaction()]).unwrap();

    assert!(chain.verify().is_ok());
}

#[test]
fn test_chain_invalid_link_detection() {
    let mut chain = Chain::new(1, None).unwrap();

    // Create a block with wrong previous_hash
    let invalid_block = Block::new(
        vec![create_test_transaction()],
        [1; 32], // Wrong previous hash
        1,
    )
    .unwrap();

    // Add block directly to bypass normal addition
    chain.blocks.push(invalid_block);

    assert!(matches!(chain.verify(), Err(ChainError::InvalidBlockLink)));
}

#[test]
fn test_latest_block() {
    let mut chain = create_test_chain(None, None).unwrap();
    let transactions = vec![create_test_transaction()];

    chain.add_block(transactions.clone()).unwrap();

    let latest = chain.latest_block().unwrap();
    assert_eq!(latest.transactions(), &transactions);
}

#[test]
fn test_multiple_blocks_addition() {
    let mut chain = create_test_chain(None, None).unwrap();
    let block_count = 5;

    for i in 0..block_count {
        let transactions = vec![create_test_transaction()];
        chain.add_block(transactions.clone()).unwrap();

        // Verify each block individually
        let block = chain.get_block(i + 1).unwrap();
        assert!(
            block.has_valid_proof(),
            "Block {} should have valid proof",
            i
        );

        // Verify chain after each addition
        assert!(
            chain.verify().is_ok(),
            "Chain should be valid after block {}",
            i
        );
    }

    assert_eq!(chain.len(), block_count + 1);
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

    chain.add_block(vec![create_test_transaction()]).unwrap();

    let block = chain.latest_block().unwrap();
    assert_eq!(block.difficulty(), difficulty);
}

#[test]
fn test_chain_tamper_detection() {
    let mut chain = create_test_chain(None, None).unwrap();
    println!("\nInitial genesis block hash: {:?}", chain.blocks[0].hash());
    
    // Create a block and add it
    let original_tx = Transaction::new(
        create_test_address(1),
        create_test_address(2),
        100,
        1,
    );
    chain.add_block(vec![original_tx.clone()]).unwrap();
    
    println!("\nAfter adding original block:");
    println!("Original tx: {:?}", original_tx);
    println!("Block hash before tampering: {:?}", chain.blocks.last().unwrap().hash());
    
    // Remember the original hash
    let original_hash = chain.blocks.last().unwrap().hash();

    // Tamper with the block
    let tampered_tx = Transaction::new(
        create_test_address(99),
        create_test_address(100),
        999999,
        2,
    );
    println!("\nTrying to tamper with:");
    println!("Tampered tx: {:?}", tampered_tx);

    if let Some(block) = chain.blocks.last_mut() {
        block.set_transactions_for_testing(vec![tampered_tx]);
        println!("\nAfter tampering:");
        println!("New block hash: {:?}", block.hash());
        println!("Hash changed: {}", block.hash() != original_hash);
    }

    println!("\nVerification result: {:?}", chain.verify());
    assert!(chain.verify().is_err());
}

#[test]
fn test_invalid_genesis_detection() {
    let genesis_tx1 = Transaction::new(create_test_address(1), create_test_address(1), 0, 0);

    let genesis_tx2 = Transaction::new(create_test_address(2), create_test_address(2), 0, 0);

    let chain1 = create_test_chain(Some(1), Some(genesis_tx1)).unwrap();
    let chain2 = create_test_chain(Some(1), Some(genesis_tx2)).unwrap();

    assert_ne!(
        chain1.get_block(0).unwrap().hash(),
        chain2.get_block(0).unwrap().hash()
    );
}
