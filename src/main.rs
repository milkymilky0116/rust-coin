use sha256::digest;

struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn get_last_hash(&self) -> String {
        if !self.blocks.is_empty() {
            let last_block = self.blocks.last().unwrap();
            last_block.hash.to_string()
        } else {
            "".to_string()
        }
    }

    fn add_block(&mut self, data: String) {
        let last_hash = self.get_last_hash();
        let mut new_block = Block {
            data,
            hash: "".to_string(),
            prev_hash: last_hash,
        };

        let hash_data = digest(format!("{}{}", new_block.hash, new_block.prev_hash));
        new_block.hash = hash_data;

        self.blocks.push(new_block);
    }

    fn list_block(&self) {
        self.blocks.iter().for_each(|block| {
            println!("Data: {}", block.data);

            println!("Hash: {}", block.hash);
            println!("Prev Hash: {} \n", block.prev_hash);
        })
    }

    fn new() -> BlockChain {
        BlockChain { blocks: vec![] }
    }
}

struct Block {
    data: String,
    hash: String,
    prev_hash: String,
}

fn main() {
    let mut chain = BlockChain::new();
    chain.add_block("Genesis Block".to_string());

    chain.add_block("Second Block".to_string());

    chain.add_block("Third Block".to_string());

    chain.add_block("Fourth Block".to_string());

    chain.list_block();
}
