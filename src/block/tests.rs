use super::*;
use test_case::test_case;
use crate::transaction::{Address, Transaction};
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn create_test_address(value: u8) -> Address {
    let bytes = [value; 20];
    Address::new(bytes)
}

fn create_test_transaction() -> Transaction {
    let nonce = NONCE_COUNTER.fetch_add(1, Ordering::SeqCst);
    Transaction::new(
        create_test_address(1),  // sender
        create_test_address(2),  // receiver
        100,                     // amount
        nonce,                   // unique nonce for each test transaction
    )
}

fn create_test_block(difficulty: u32) -> Block {
    let transactions = vec![create_test_transaction()];
    let previous_hash = [0; 32];
    Block::new(transactions, previous_hash, difficulty).unwrap()
}

#[test]
fn test_new_block_creation() {
    let transactions = vec![create_test_transaction()];
    let previous_hash = [0; 32];
    let difficulty = 1;
    
    let block = Block::new(transactions.clone(), previous_hash, difficulty).unwrap();
    
    assert_eq!(block.transactions(), &transactions);
    assert_eq!(block.previous_hash(), previous_hash);
    assert_ne!(block.hash(), [0; 32]); // Hash should not be empty
    assert_eq!(block.difficulty(), difficulty);
    assert_eq!(block.nonce(), 0); // Initial nonce should be 0
}

#[test]
fn test_block_hash_calculation() {
    let block = create_test_block(1);
    let calculated_hash = block.calculate_hash();
    
    assert_eq!(block.hash(), calculated_hash);
}

#[test]
fn test_mining_with_low_difficulty() {
    let mut block = create_test_block(1); // Only 1 leading zero bit required
    block.mine();
    assert!(block.has_valid_proof());
    assert!(block.verify(false).is_ok());
}

#[test]
fn test_mining_verify_fails_with_tampered_transaction() {
    let mut block = create_test_block(1);
    block.mine();
    
    // Create a different transaction
    let tampered_transaction = Transaction::new(
        create_test_address(3),  // different sender
        create_test_address(4),  // different receiver
        200,                     // different amount
        2,                       // different nonce
    );
    
    let _ = std::mem::replace(&mut block.transactions, vec![tampered_transaction]);
    
    assert!(block.verify(false).is_err());
}

#[test]
fn test_mining_multiple_difficulty_levels() {
    for difficulty in [1, 8, 16] {  // Test different difficulties
        let mut block = create_test_block(difficulty);
        block.mine();
        assert!(block.has_valid_proof(), "Failed for difficulty {}", difficulty);
        assert!(block.verify(false).is_ok(), "Verification failed for difficulty {}", difficulty);
    }
}

#[test]
fn test_proof_validation() {
    let mut block = create_test_block(8); // Require one byte of zeros
    block.mine();
    
    // The first byte should be zero
    assert_eq!(block.hash()[0], 0);
    assert!(block.has_valid_proof());
}

#[test]
fn test_nonce_increases_during_mining() {
    let mut block = create_test_block(4); // Increased difficulty to ensure nonce changes
    let initial_nonce = block.nonce();
    block.mine();
    assert!(block.nonce() > initial_nonce);
}

#[test]
fn test_empty_transactions_rejected() {
    let empty_transactions: Vec<Transaction> = vec![];
    let previous_hash = [0; 32];
    let difficulty = 1;
    
    let result = Block::new(empty_transactions, previous_hash, difficulty);
    assert!(matches!(result, Err(BlockError::EmptyTransactions)));
}

#[test_case(&[0; 32])]
#[test_case(&[1; 32])]
fn test_different_previous_hashes(prev_hash: &[u8; 32]) {
    let transactions = vec![create_test_transaction()];
    let difficulty = 1;
    let block = Block::new(transactions, *prev_hash, difficulty).unwrap();
    
    assert_eq!(block.previous_hash(), *prev_hash);
}

#[test]
fn test_block_timestamp() {
    let block = create_test_block(1);
    let now = OffsetDateTime::now_utc();
    
    // Block timestamp should be close to now
    // Allow 1 second difference to account for test execution time
    assert!((block.timestamp().unix_timestamp() - now.unix_timestamp()).abs() <= 1);
}

#[test]
fn test_mining_resets_hash() {
    let mut block = create_test_block(4); // Increased difficulty to ensure hash changes
    let initial_hash = block.hash();
    block.mine();
    
    // Hash should be different after mining
    assert_ne!(block.hash(), initial_hash);
    assert!(block.has_valid_proof());
}

#[test]
fn test_verify_checks_both_hash_and_proof() {
    let mut block = create_test_block(4); // decent difficulty
    block.mine();
    assert!(block.verify(false).is_ok(), "Initial valid state failed");

    // Test 1: Invalid hash (modify transaction)
    let mut invalid_block = block.clone();
    invalid_block.transactions[0] = create_test_transaction(); // modify transaction
    assert!(matches!(invalid_block.verify(false), Err(BlockError::InvalidHash)), 
        "Modified transaction should cause invalid hash");

    // Test 2: Invalid proof (valid hash but doesn't meet difficulty)
    let invalid_block = Block::new(
        vec![create_test_transaction()],
        [0; 32],
        block.difficulty(),
    ).unwrap();
    // Don't mine it, so it won't meet proof of work
    assert!(matches!(invalid_block.verify(false), Err(BlockError::InvalidProofOfWork)),
        "Unmined block should fail proof of work");

    // Test 3: Verify original block still valid
    assert!(block.verify(false).is_ok(), "Original block should remain valid");
}