use crate::transaction::Address;

#[derive(Debug, Clone)]
pub struct UTXO {
    utxo_id: [u8; 32],  // Unique identifier for this UTXO
    owner: Address,     // Who can spend this UTXO
    amount: u64,        // How much it's worth
    block_created: u64, // Which block created this UTXO
    tx_index: u32,      // Position of tx in block
}

impl UTXO {
  pub fn new(
      utxo_id: [u8; 32],
      owner: Address,
      amount: u64,
      block_created: u64,
      tx_index: u32,
  ) -> Self {
      Self {
          utxo_id,
          owner,
          amount,
          block_created,
          tx_index,
      }
  }

  // Getters
  pub fn utxo_id(&self) -> &[u8; 32] {
      &self.utxo_id
  }

  pub fn owner(&self) -> &Address {
      &self.owner
  }

  pub fn amount(&self) -> u64 {
      self.amount
  }

  pub fn block_created(&self) -> u64 {
      self.block_created
  }

  pub fn tx_index(&self) -> u32 {
      self.tx_index
  }
}

#[cfg(test)]
mod tests;