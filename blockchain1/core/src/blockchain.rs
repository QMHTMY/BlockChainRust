use crate::block::Block;

const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain { blocks: vec![Self::genesis_block()] }
    }

    fn genesis_block() -> Block {
        Block::new("创始区块".to_string(), PRE_HASH.to_string())
    }

    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash  = pre_block.hash.clone();
        let new_block = Block::new(data, pre_hash);
        self.blocks.push(new_block);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
