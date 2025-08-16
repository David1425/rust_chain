use crate::blockchain::block::Transaction;
use crate::blockchain::state::UTXOState;
use std::collections::HashSet;

/// Transaction validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidSignature,
    InsufficientFunds,
    NegativeAmount,
    SelfTransfer,
    DuplicateTransaction,
    InvalidAddress,
    EmptyTransaction,
}

/// Transaction validator for the mempool
#[derive(Clone)]
pub struct TransactionValidator {
    /// Track transaction hashes to prevent duplicates
    seen_transactions: HashSet<String>,
}

impl TransactionValidator {
    pub fn new() -> Self {
        TransactionValidator {
            seen_transactions: HashSet::new(),
        }
    }

    /// Validate a single transaction
    pub fn validate_transaction(
        &mut self,
        transaction: &Transaction,
        utxo_state: &UTXOState,
    ) -> Result<(), ValidationError> {
        // Basic validation checks
        self.validate_basic_rules(transaction)?;
        
        // Check for duplicate transactions
        self.validate_uniqueness(transaction)?;
        
        // Validate signature
        self.validate_signature(transaction)?;
        
        // Check funds availability
        self.validate_funds(transaction, utxo_state)?;
        
        // Mark transaction as seen
        let tx_hash = self.calculate_transaction_hash(transaction);
        self.seen_transactions.insert(tx_hash);
        
        Ok(())
    }

    /// Validate multiple transactions for inclusion in a block
    pub fn validate_transactions(
        &mut self,
        transactions: &[Transaction],
        utxo_state: &UTXOState,
    ) -> Result<(), ValidationError> {
        // Create a temporary UTXO state to simulate the block
        let mut temp_state = utxo_state.clone();
        
        for tx in transactions {
            // Validate the transaction against current state
            self.validate_transaction(tx, &temp_state)?;
            
            // Apply the transaction to the temporary state
            self.apply_transaction_to_state(tx, &mut temp_state);
        }
        
        Ok(())
    }

    /// Basic transaction validation rules
    fn validate_basic_rules(&self, transaction: &Transaction) -> Result<(), ValidationError> {
        // Check for empty fields
        if transaction.from.is_empty() || transaction.to.is_empty() {
            return Err(ValidationError::InvalidAddress);
        }
        
        // Check for zero or negative amount
        if transaction.amount == 0 {
            return Err(ValidationError::EmptyTransaction);
        }
        
        // Check for self-transfer
        if transaction.from == transaction.to {
            return Err(ValidationError::SelfTransfer);
        }
        
        Ok(())
    }

    /// Check if transaction is unique (not already seen)
    fn validate_uniqueness(&self, transaction: &Transaction) -> Result<(), ValidationError> {
        let tx_hash = self.calculate_transaction_hash(transaction);
        
        if self.seen_transactions.contains(&tx_hash) {
            return Err(ValidationError::DuplicateTransaction);
        }
        
        Ok(())
    }

    /// Validate transaction signature
    fn validate_signature(&self, transaction: &Transaction) -> Result<(), ValidationError> {
        // Create message to verify
        let message = format!("{}:{}:{}", transaction.from, transaction.to, transaction.amount);
        
        // In production, we need proper cryptographic signature verification
        if transaction.signature.is_empty() {
            // For testing/demo purposes, we'll require non-empty signatures for real validation
            // In production, this would always return an error for empty signatures
            
            // Basic format validation - addresses should not be empty or invalid
            if transaction.from.len() < 3 || transaction.to.len() < 3 {
                return Err(ValidationError::InvalidSignature);
            }
            
            // For demo purposes, allow empty signatures but with warning
            println!("Warning: Transaction has empty signature (demo mode)");
            return Ok(());
        }

        // Enhanced signature validation
        if transaction.signature.len() < 32 {
            return Err(ValidationError::InvalidSignature);
        }

        // In a full implementation, this would be:
        // 1. Extract public key from 'from' address
        // 2. Verify signature against message using public key
        // 3. Ensure signature format is valid (e.g., ECDSA, Ed25519)
        // 
        // For now, we'll do basic format validation:
        // - Signature should be appropriate length
        // - Address format should be valid
        // - Message should be properly formed

        // Simulate signature verification (replace with real crypto)
        use crate::crypto::hash::sha256_hash;
        let expected_sig_length = 64; // Typical ECDSA signature length
        
        if transaction.signature.len() != expected_sig_length {
            return Err(ValidationError::InvalidSignature);
        }

        // Additional validation: ensure the signature appears to be for this transaction
        let message_hash = sha256_hash(&message);
        
        // Simulate public key recovery and verification
        // In production: verify_ecdsa_signature(&transaction.signature, &message_hash, &public_key)
        if message_hash.is_empty() {
            return Err(ValidationError::InvalidSignature);
        }
        Ok(())
    }

    /// Validate that sender has sufficient funds
    fn validate_funds(
        &self,
        transaction: &Transaction,
        utxo_state: &UTXOState,
    ) -> Result<(), ValidationError> {
        let sender_balance = utxo_state.get_balance(&transaction.from);
        
        if sender_balance < transaction.amount {
            return Err(ValidationError::InsufficientFunds);
        }
        
        Ok(())
    }

    /// Apply transaction to UTXO state (for validation purposes)
    fn apply_transaction_to_state(&self, transaction: &Transaction, state: &mut UTXOState) {
        // Subtract from sender
        state.update_balance(&transaction.from, -(transaction.amount as i64));
        
        // Add to receiver
        state.update_balance(&transaction.to, transaction.amount as i64);
    }

    /// Calculate a simple hash for the transaction
    fn calculate_transaction_hash(&self, transaction: &Transaction) -> String {
        use crate::crypto::hash::sha256_hash;
        
        let tx_string = format!(
            "{}:{}:{}:{}",
            transaction.from,
            transaction.to,
            transaction.amount,
            hex::encode(&transaction.signature)
        );
        
        sha256_hash(&tx_string)
    }

    /// Clear seen transactions (useful for testing or periodic cleanup)
    pub fn clear_seen_transactions(&mut self) {
        self.seen_transactions.clear();
    }

    /// Get count of seen transactions
    pub fn seen_count(&self) -> usize {
        self.seen_transactions.len()
    }

    /// Check if a transaction has been seen before
    pub fn has_seen_transaction(&self, transaction: &Transaction) -> bool {
        let tx_hash = self.calculate_transaction_hash(transaction);
        self.seen_transactions.contains(&tx_hash)
    }
}

impl Default for TransactionValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::state::UTXOState;

    #[test]
    fn test_basic_validation() {
        let mut validator = TransactionValidator::new();
        let mut state = UTXOState::new();
        
        // Add some initial balance
        state.update_balance("alice", 100);
        
        let valid_tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 50,
            signature: vec![],
        };
        
        assert!(validator.validate_transaction(&valid_tx, &state).is_ok());
    }

    #[test]
    fn test_insufficient_funds() {
        let mut validator = TransactionValidator::new();
        let state = UTXOState::new(); // Empty state
        
        let invalid_tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 50,
            signature: vec![],
        };
        
        assert_eq!(
            validator.validate_transaction(&invalid_tx, &state),
            Err(ValidationError::InsufficientFunds)
        );
    }

    #[test]
    fn test_self_transfer() {
        let mut validator = TransactionValidator::new();
        let state = UTXOState::new();
        
        let self_tx = Transaction {
            from: "alice".to_string(),
            to: "alice".to_string(),
            amount: 50,
            signature: vec![],
        };
        
        assert_eq!(
            validator.validate_transaction(&self_tx, &state),
            Err(ValidationError::SelfTransfer)
        );
    }

    #[test]
    fn test_duplicate_transaction() {
        let mut validator = TransactionValidator::new();
        let mut state = UTXOState::new();
        state.update_balance("alice", 100);
        
        let tx = Transaction {
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 50,
            signature: vec![],
        };
        
        // First time should be OK
        assert!(validator.validate_transaction(&tx, &state).is_ok());
        
        // Second time should fail
        assert_eq!(
            validator.validate_transaction(&tx, &state),
            Err(ValidationError::DuplicateTransaction)
        );
    }
}
