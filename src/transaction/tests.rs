use super::*;

fn create_test_address(value: u8) -> Address {
    let bytes = [value; 20];
    Address::new(bytes)
}

fn create_test_transaction() -> Transaction {
    Transaction::new(
        create_test_address(1),  // sender
        create_test_address(2),  // receiver
        100,                     // amount
        1,                       // nonce
    )
}

#[test]
fn test_transaction_creation() {
    let sender = create_test_address(1);
    let receiver = create_test_address(2);
    let amount = 100;
    let nonce = 1;

    let transaction = Transaction::new(sender.clone(), receiver.clone(), amount, nonce);

    assert_eq!(transaction.sender(), &sender);
    assert_eq!(transaction.receiver(), &receiver);
    assert_eq!(transaction.amount(), amount);
    assert_eq!(transaction.nonce(), nonce);
}

#[test]
fn test_valid_transaction() {
    let transaction = create_test_transaction();
    assert!(transaction.validate(false).is_ok());
}

#[test]
fn test_zero_amount_transaction() {
    let transaction = Transaction::new(
        create_test_address(1),
        create_test_address(2),
        0,  // Invalid amount
        1,
    );
    assert!(matches!(
        transaction.validate(false),
        Err(TransactionError::InvalidAmount)
    ));
}

#[test]
fn test_same_sender_receiver() {
    let address = create_test_address(1);
    let transaction = Transaction::new(
        address.clone(),
        address,
        100,
        1,
    );
    assert!(matches!(
        transaction.validate(false),
        Err(TransactionError::SameSenderReceiver)
    ));
}

#[test]
fn test_address_equality() {
    let address1 = create_test_address(1);
    let address2 = create_test_address(1);
    let address3 = create_test_address(2);

    assert_eq!(address1, address2);
    assert_ne!(address1, address3);
}

#[test]
fn test_transaction_uniqueness() {
    let tx1 = create_test_transaction();
    let tx2 = Transaction::new(
        create_test_address(1),
        create_test_address(2),
        100,
        2,  // Different nonce
    );

    assert_ne!(tx1, tx2);
}