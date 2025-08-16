use std::collections::HashMap;
use crate::crypto::hash::H256;
use crate::transaction::TxOutput;

pub struct State {
    utxos: HashMap<H256, TxOutput>, // Maps TXID + output index to unspent outputs
}

impl State {
    pub fn new() -> Self {
        State { utxos: HashMap::new() }
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<()> {
        // Remove spent UTXOs, add new ones
    }
}