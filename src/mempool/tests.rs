use super::*;
use crate::transaction::{Transaction, Address};

fn create_test_address(value: u8) -> Address {
    Address::new([value; 20])
}

fn create_test_transaction(nonce: u64) -> Transaction {
    Transaction::new(
        create_test_address(1),  // sender
        create_test_address(2),  // receiver
        100,                     // amount
        nonce,                   // unique nonce for each test
    )
}

#[test]
fn test_mempool_new() {
    let mempool = Mempool::new();
    assert!(mempool.is_empty());
    assert_eq!(mempool.len(), 0);
}

#[test]
fn test_add_transaction() {
    let mut mempool = Mempool::new();
    let tx = create_test_transaction(1);
    
    mempool.add_transaction(tx.clone()).unwrap();
    assert_eq!(mempool.len(), 1);
    assert!(mempool.contains(&tx));
}

#[test]
fn test_duplicate_transaction() {
    let mut mempool = Mempool::new();
    let tx = create_test_transaction(1);
    
    mempool.add_transaction(tx.clone()).unwrap();
    assert!(matches!(
        mempool.add_transaction(tx.clone()),
        Err(MempoolError::DuplicateTransaction)
    ));
}

#[test]
fn test_get_transactions_for_block() {
    let mut mempool = Mempool::new();
    
    // Add transactions with different nonces
    let tx1 = create_test_transaction(1);
    let tx2 = create_test_transaction(2);
    let tx3 = create_test_transaction(3);
    
    mempool.add_transaction(tx1.clone()).unwrap();
    mempool.add_transaction(tx2.clone()).unwrap();
    mempool.add_transaction(tx3.clone()).unwrap();
    
    // Should get transactions in nonce order
    let selected = mempool.get_transactions(2); // Get 2 transactions
    assert_eq!(selected.len(), 2);

    // println!("Selected: {:#?}", selected);
    // println!("Expected: {:#?}", vec![tx1, tx2]);

    assert_eq!(selected[0], tx1);
    assert_eq!(selected[1], tx2);
}

#[test]
fn test_remove_transactions() {
    let mut mempool = Mempool::new();
    let tx1 = create_test_transaction(1);
    let tx2 = create_test_transaction(2);
    
    mempool.add_transaction(tx1.clone()).unwrap();
    mempool.add_transaction(tx2.clone()).unwrap();
    
    mempool.remove_transactions(&[tx1.clone()]);
    assert_eq!(mempool.len(), 1);
    assert!(!mempool.contains(&tx1));
    assert!(mempool.contains(&tx2));
}

#[test]
fn test_clear_mempool() {
    let mut mempool = Mempool::new();
    mempool.add_transaction(create_test_transaction(1)).unwrap();
    mempool.add_transaction(create_test_transaction(2)).unwrap();
    
    mempool.clear();
    assert!(mempool.is_empty());
}

#[test]
fn test_transaction_ordering() {
    let mut mempool = Mempool::new();
    
    // Add transactions in random order
    let tx3 = create_test_transaction(3);
    let tx1 = create_test_transaction(1);
    let tx2 = create_test_transaction(2);
    
    mempool.add_transaction(tx3.clone()).unwrap();
    mempool.add_transaction(tx1.clone()).unwrap();
    mempool.add_transaction(tx2.clone()).unwrap();
    
    // Should get all transactions in nonce order
    let transactions = mempool.get_transactions(3);
    assert_eq!(transactions, vec![tx1, tx2, tx3]);
}