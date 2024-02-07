use serde::Serialize;
use sha256::digest;

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    pub data: String,
    pub hash: String,
    pub prev_hash: String,
    pub height: usize,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            data: "".to_string(),
            hash: "".to_string(),
            prev_hash: "".to_string(),
            height: 0,
        }
    }
}

impl Block {
    pub fn calculate_hash(&self, last_hash: &String) -> String {
        let data = format!("{}{}", self.data, last_hash);
        digest(data)
    }
    pub fn create_block(data: String, prev_hash: String, height: usize) -> Block {
        let mut block = Block {
            data,
            hash: "".to_string(),
            prev_hash,
            height,
        };
        block.hash = block.calculate_hash(&prev_hash);
        block
    }
}
