use sha256::digest;

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn get_last_hash(&self) -> String {
        if !self.blocks.is_empty() {
            let last_block = self.blocks.last().unwrap();
            last_block.hash.to_string()
        } else {
            "".to_string()
        }
    }
    pub fn add_block(&mut self, data: String) {
        let mut new_block = Block::default();
        new_block.create_block(data, self.get_last_hash());
        self.blocks.push(new_block);
    }

    pub fn all_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn init(&mut self) {
        self.add_block("Genesis Block".to_string());
    }

    pub const fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }
}

#[derive(Debug)]
pub struct Block {
    pub data: String,
    pub hash: String,
    pub prev_hash: String,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            data: "".to_string(),
            hash: "".to_string(),
            prev_hash: "".to_string(),
        }
    }
}

impl Block {
    pub fn calculate_hash(&self, last_hash: &String) -> String {
        let data = format!("{}{}", self.data, last_hash);
        digest(data)
    }
    pub fn create_block(&mut self, data: String, last_hash: String) {
        self.data = data;
        self.hash = self.calculate_hash(&last_hash);
        self.prev_hash = last_hash;
    }
}
