use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UTXO {
	pub owner: String,
	pub amount: u64,
}

#[derive(Debug, Default)]
pub struct State {
	pub utxos: HashMap<String, UTXO>, // key: tx_output_id
}

impl State {
	pub fn new() -> Self {
		State { utxos: HashMap::new() }
	}

	pub fn add_utxo(&mut self, id: String, utxo: UTXO) {
		self.utxos.insert(id, utxo);
	}

	pub fn spend_utxo(&mut self, id: &str) -> Option<UTXO> {
		self.utxos.remove(id)
	}

	pub fn get_balance(&self, owner: &str) -> u64 {
		self.utxos.values().filter(|u| u.owner == owner).map(|u| u.amount).sum()
	}
}

/// Simplified UTXO state for transaction validation
#[derive(Debug, Default, Clone)]
pub struct UTXOState {
    balances: HashMap<String, u64>,
}

impl UTXOState {
    pub fn new() -> Self {
        UTXOState {
            balances: HashMap::new(),
        }
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Update balance by a delta (can be negative)
    pub fn update_balance(&mut self, address: &str, delta: i64) {
        let current_balance = self.get_balance(address) as i64;
        let new_balance = (current_balance + delta).max(0) as u64;
        
        if new_balance == 0 {
            self.balances.remove(address);
        } else {
            self.balances.insert(address.to_string(), new_balance);
        }
    }

    /// Set balance directly
    pub fn set_balance(&mut self, address: &str, balance: u64) {
        if balance == 0 {
            self.balances.remove(address);
        } else {
            self.balances.insert(address.to_string(), balance);
        }
    }

    /// Get all addresses with balances
    pub fn get_all_balances(&self) -> &HashMap<String, u64> {
        &self.balances
    }

    /// Clear all balances
    pub fn clear(&mut self) {
        self.balances.clear();
    }
}
