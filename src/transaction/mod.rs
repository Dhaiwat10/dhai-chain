use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Invalid amount: amount must be greater than 0")]
    InvalidAmount,
    #[error("Invalid address format")]
    InvalidAddress,
    #[error("Sender and receiver cannot be the same")]
    SameSenderReceiver,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Address([u8; 20]); // 20 bytes address like Ethereum

impl Address {
    pub fn new(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    sender: Address,
    receiver: Address,
    amount: u64,
    nonce: u64, // To prevent replay attacks
}

impl Transaction {
    pub fn new(sender: Address, receiver: Address, amount: u64, nonce: u64) -> Self {
        Self {
            sender,
            receiver,
            amount,
            nonce,
        }
    }

    pub fn validate(&self, is_genesis: bool) -> Result<(), TransactionError> {
        if self.amount == 0 {
            return Err(TransactionError::InvalidAmount);
        }

        if !is_genesis && self.sender == self.receiver {
            return Err(TransactionError::SameSenderReceiver);
        }

        Ok(())
    }

    pub fn sender(&self) -> &Address {
        &self.sender
    }

    pub fn receiver(&self) -> &Address {
        &self.receiver
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}

#[cfg(test)]
mod tests;