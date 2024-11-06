use super::*;

fn create_test_address() -> Address {
   Address::new([1; 20])
}

fn create_test_utxo_id() -> [u8; 32] {
   [1; 32]
}

#[test]
fn test_utxo_creation() {
   let utxo_id = create_test_utxo_id();
   let owner = create_test_address();
   let amount = 100;
   let block_created = 1;
   let tx_index = 0;

   let utxo = UTXO::new(
       utxo_id,
       owner.clone(),
       amount,
       block_created,
       tx_index,
   );

   assert_eq!(utxo.utxo_id(), &utxo_id);
   assert_eq!(utxo.owner(), &owner);
   assert_eq!(utxo.amount(), amount);
   assert_eq!(utxo.block_created(), block_created);
   assert_eq!(utxo.tx_index(), tx_index);
}
