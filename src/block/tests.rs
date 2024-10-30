use super::*;
use test_case::test_case;

fn create_test_block(difficulty: u32) -> Block {
    let data = b"test data".to_vec();
    let previous_hash = [0; 32];
    Block::new(data, previous_hash, difficulty)
}

#[test]
fn test_new_block_creation() {
    let data = b"test data".to_vec();
    let previous_hash = [0; 32];
    let difficulty = 1;
    
    let block = Block::new(data.clone(), previous_hash, difficulty);
    
    assert_eq!(block.data(), &data);
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
    assert!(block.verify().is_ok());
}

#[test]
fn test_mining_verify_fails_with_tampered_data() {
    let mut block = create_test_block(1);
    block.mine();
    
    // Tamper with data after mining
    let tampered_data = b"tampered data".to_vec();
    let _ = std::mem::replace(&mut block.data, tampered_data);
    
    assert!(block.verify().is_err());
}

#[test]
fn test_mining_multiple_difficulty_levels() {
    for difficulty in [1, 8, 16] {  // Test different difficulties
        let mut block = create_test_block(difficulty);
        block.mine();
        assert!(block.has_valid_proof(), "Failed for difficulty {}", difficulty);
        assert!(block.verify().is_ok(), "Verification failed for difficulty {}", difficulty);
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
    let mut block = create_test_block(1);
    let initial_nonce = block.nonce();
    block.mine();
    print!("{:?}", block.nonce());
    assert!(block.nonce() > initial_nonce);
}

#[test_case(&[0; 32])]
#[test_case(&[1; 32])]
fn test_different_previous_hashes(prev_hash: &[u8; 32]) {
    let data = b"test data".to_vec();
    let difficulty = 1;
    let block = Block::new(data, *prev_hash, difficulty);
    
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
    let mut block = create_test_block(1);
    let initial_hash = block.hash();
    block.mine();
    
    // Hash should be different after mining
    assert_ne!(block.hash(), initial_hash);
}

#[test]
fn test_verify_checks_both_hash_and_proof() {
    let mut block = create_test_block(1);
    block.mine();
    
    // Save valid state
    let valid_hash = block.hash;
    let valid_nonce = block.nonce;
    
    // Test 1: Invalid hash
    block.hash = [0; 32];
    assert!(matches!(block.verify(), Err(BlockError::InvalidHash)));
    
    // Test 2: Invalid proof
    block.hash = valid_hash;
    block.nonce = 0; // Reset nonce to make proof invalid
    assert!(matches!(block.verify(), Err(BlockError::InvalidHash)));
    
    // Test 3: Valid state
    block.nonce = valid_nonce;
    block.hash = block.calculate_hash();
    assert!(block.verify().is_ok());
}