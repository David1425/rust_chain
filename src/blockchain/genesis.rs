use crate::blockchain::block::{Block, Transaction};

pub fn genesis_block() -> Block {
	let tx1 = Transaction {
		from: "genesis".to_string(),
		to: "alice".to_string(),
		amount: 1000,
		signature: vec![],
	};
	
	let tx2 = Transaction {
		from: "genesis".to_string(),
		to: "bob".to_string(),
		amount: 500,
		signature: vec![],
	};
	
	Block::new(
		"0".to_string(),
		vec![tx1, tx2],
		0,
		0,
		0, // Genesis block is at height 0
	)
}
