use crate::block;
use super::*;
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String)->Result<()> {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let new_block = block::Block::new_block(data, pre_block.header.tx_hash.clone())?;
        self.blocks.push(new_block);
        Ok(())
    }

fn new_genesis_block() -> block::Block {
        block::Block::new_block("this is genesis block".to_string(), String::from("")).unwrap()
    }

    pub fn new_blockchain()->BlockChain{
        BlockChain{
            blocks:vec![BlockChain::new_genesis_block()]
        }
    }
}