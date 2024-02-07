use super::block::Block;

#[derive(Debug)]
pub struct BlockChain {
    newest_hash: String,
    height: usize,
}

impl BlockChain {
    pub fn add_block(&mut self, data: String) {
        let block = Block::create_block(data, self.newest_hash, self.height);
        self.newest_hash = block.hash;
        self.height = block.height;
    }

    pub fn init(&mut self) {
        self.add_block("Genesis Block".to_string());
    }

    pub const fn new() -> BlockChain {
        BlockChain {
            newest_hash: "".to_string(),
            height: 0,
        }
    }
}
