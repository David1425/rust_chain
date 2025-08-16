use crate::blockchain::block::Block;
use crate::blockchain::genesis::genesis_block;

pub struct Chain {
	pub blocks: Vec<Block>,
}

impl Chain {
	pub fn new() -> Self {
		Chain { blocks: vec![genesis_block()] }
	}

	pub fn add_block(&mut self, block: Block) -> bool {
		if self.validate_block(&block) {
			self.blocks.push(block);
			true
		} else {
			false
		}
	}

	pub fn validate_block(&self, block: &Block) -> bool {
		let last_hash = self.blocks.last().map(|b| b.header.hash.clone()).unwrap_or_default();
		let expected_height = self.blocks.len() as u64;
		block.header.previous_hash == last_hash && block.header.height == expected_height
	}
}
