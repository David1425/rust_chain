use crate::blockchain::block::{Block, Transaction};

pub fn genesis_block() -> Block {
	let tx = Transaction {
		from: "0".to_string(),
		to: "genesis_address".to_string(),
		amount: 50,
		signature: vec![],
	};
	Block::new(
		"0".to_string(),
		vec![tx],
		0,
		0,
		0, // Genesis block is at height 0
	)
}
