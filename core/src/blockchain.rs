use crate::block;

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<block::Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        println!("add block data = {}", data);
        let pre_hash = self.blocks.last().unwrap();
        let block = block::Block::new_block(data, pre_hash.hash.clone());
        self.blocks.push(block);
    }

    fn new_genesis_block() -> block::Block {
       block::Block::new_block("初始区块".to_string(), "".to_string())
    }

    pub fn new_blockchain() -> Self {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}
