use std::collections::{HashMap, BinaryHeap};

pub struct Mempool {
    txs: HashMap<H256, Transaction>,
    by_fee: BinaryHeap<Transaction>, // Higher fees first
}

impl Mempool {
    pub fn add_tx(&mut self, tx: Transaction) -> bool {
        if self.validator.check(&tx) {
            self.txs.insert(tx.hash(), tx);
            true
        } else {
            false
        }
    }
}