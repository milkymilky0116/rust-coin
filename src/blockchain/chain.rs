pub struct BlockChain {
    pub blocks: Vec<Block>,
}
impl BlockChain {
    pub fn get_last_hash(&self) -> String {
        if !self.blocks.is_empty() {
            let last_block = self.blocks.last().unwrap();
        }
        "".to_string()
    }
    pub const fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }
}

pub struct Block {
    pub data: String,
    pub hash: String,
    pub prev_hash: String,
}
