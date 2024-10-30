use super::*;

fn create_test_chain(difficulty: Option<u32>, genesis_data: Option<Vec<u8>>) -> Chain {
    let difficulty = difficulty.unwrap_or(1);
    Chain::new(difficulty, genesis_data) // Use low difficulty for faster tests
}

#[test]
fn test_new_chain_creation() {
    let chain = create_test_chain(None, None);
    
    assert_eq!(chain.len(), 1); // Should have genesis block
    assert_eq!(chain.current_difficulty(), 1);
    
    // Verify genesis block
    let genesis = chain.get_block(0).unwrap();
    assert_eq!(genesis.previous_hash(), [0; 32]);
    assert!(!chain.is_empty());
}

#[test]
fn test_adding_block() {
    let mut chain = create_test_chain(None, None);
    let data = b"Test Block".to_vec();
    
    chain.add_block(data.clone()).unwrap();
    
    assert_eq!(chain.len(), 2);
    let block = chain.get_block(1).unwrap();
    assert_eq!(block.data(), &data);
    
    // Verify block links
    let genesis = chain.get_block(0).unwrap();
    assert_eq!(block.previous_hash(), genesis.hash());
}

#[test]
fn test_chain_verification() {
    let mut chain = create_test_chain(None, None);
    
    // Add a few blocks
    chain.add_block(b"Block 1".to_vec()).unwrap();
    chain.add_block(b"Block 2".to_vec()).unwrap();

    print!("{:?}", chain.blocks);
    
    assert!(chain.verify().is_ok());
}

#[test]
fn test_chain_invalid_link_detection() {
    let mut chain = Chain::new(1, None);
    
    // Create a block with wrong previous_hash
    let invalid_block = Block::new(
        b"Invalid Block".to_vec(),
        [1; 32], // Wrong previous hash
        1,
    );
    
    // Add block directly to bypass normal addition
    chain.blocks.push(invalid_block);
    
    assert!(matches!(chain.verify(), Err(ChainError::InvalidBlockLink)));
}

#[test]
fn test_latest_block() {
    let mut chain = create_test_chain(None, None);
    let data = b"Latest Block".to_vec();
    
    chain.add_block(data.clone()).unwrap();
    
    let latest = chain.latest_block().unwrap();
    assert_eq!(latest.data(), &data);
}

#[test]
fn test_multiple_blocks_addition() {
    let mut chain = create_test_chain(None, None);
    let block_count = 5;
    
    for i in 0..block_count {
        let data = format!("Block {}", i).into_bytes();
        chain.add_block(data).unwrap();
    }
    
    assert_eq!(chain.len(), block_count + 1); // +1 for genesis
    assert!(chain.verify().is_ok());
}

#[test]
fn test_empty_chain_verification() {
    let mut chain = create_test_chain(None, None);
    chain.blocks.clear(); // Force empty chain for testing
    
    assert!(matches!(chain.verify(), Err(ChainError::EmptyChain)));
}

#[test]
fn test_block_difficulty_matches_chain() {
    let difficulty = 2;
    let mut chain = Chain::new(difficulty, None);
    
    chain.add_block(b"Test Block".to_vec()).unwrap();
    
    let block = chain.latest_block().unwrap();
    assert_eq!(block.difficulty(), difficulty);
}

#[test]
fn test_chain_tamper_detection() {
    let mut chain = create_test_chain(None, None);
    chain.add_block(b"Block 1".to_vec()).unwrap();
    
    let mut tampered_chain = Chain::new(1, None); // New chain with different genesis
    tampered_chain.add_block(b"Tampered Block".to_vec()).unwrap();
    let tampered_block = tampered_chain.latest_block().unwrap();

    chain.add_block(tampered_block.data().to_vec()).unwrap();
    
    assert!(chain.verify().is_err());
}

#[test]
fn test_invalid_genesis_detection() {
    let chain1 = create_test_chain(Some(1), Some(b"Genesis Block".to_vec()));
    let chain2 = create_test_chain(Some(2), Some(b"Different Genesis Block".to_vec()));
    
    assert_ne!(
        chain1.get_block(0).unwrap().hash(),
        chain2.get_block(0).unwrap().hash()
    );
}