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
