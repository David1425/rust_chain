use rocksdb::{DB, Options};
use crate::blockchain::block::Block;
use crate::crypto::hash::H256;
use anyhow::Result;

pub struct BlockchainDB {
    db: DB,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(BlockchainDB { db })
    }

    pub fn put_block(&self, block: &Block) -> Result<()> {
        self.db.put(block.hash(), bincode::serialize(block)?)?;
        Ok(())
    }

    pub fn get_block(&self, hash: &H256) -> Result<Option<Block>> {
        match self.db.get(hash)? {
            Some(data) => Ok(Some(bincode::deserialize(&data)?)),
            None => Ok(None),
        }
    }
}