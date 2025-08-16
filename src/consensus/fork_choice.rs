use crate::blockchain::{Block, Blockchain};

pub fn select_best_chain(chains: Vec<Blockchain>) -> Blockchain {
    // Pick the chain with the most cumulative PoW (or highest stake)
    chains.into_iter().max_by_key(|chain| chain.total_work()).unwrap()
}