const DB_NAME: &str = "blockchain.db";
pub fn new() -> sled::Db {
    sled::open(DB_NAME).unwrap()
}
