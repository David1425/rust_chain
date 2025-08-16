use crate::blockchain::block::Block;

pub fn genesis_block() -> Block {
	Block::new(
		"0".to_string(),
		vec!["Genesis Transaction".to_string()],
		0,
		0,
	)
}
