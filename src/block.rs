use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::transaction::Transaction;
use std::time::SystemTime;

// Define a block structure
#[derive(Debug, Hash)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: u64,
    pub hash: u64,
}
impl Block {
    // Method to calculate the hash of the block
    pub fn calculate_hash(&self) -> u64 {
        // println!("Inside cal hash");
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    // Method to mine a block
    pub fn mine_block(&mut self, difficulty: u32) {
        println!("Inside mine hash");
        self.timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Calculate the hash before entering the mining loop
        // self.hash = self.calculate_hash();

        while self.hash > 10u64.pow(difficulty) {
            // Increment timestamp and recalculate the hash inside the loop
            self.timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            self.hash = self.calculate_hash();
            self.index += 1;
        }

        println!("Block mined: {:#?}", self);
    }
}
 