mod block;
mod transaction;

use block::Block;
use transaction::Transaction;
use std::time::SystemTime;

// Rest of the code for the main function and Blockchain struct
// Define a basic blockchain structure
#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
}

impl Blockchain {
    // Method to create a new blockchain
    fn new(difficulty: u32) -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };

        // Genesis block
        blockchain.create_block(vec![Transaction {
            sender: "Genesis".to_string(),
            recipient: "Alice".to_string(),
            amount: 100,
        }]);

        blockchain
    }

    // Method to create a new block
    fn create_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = match self.chain.last() {
            Some(last_block) => last_block.hash,
            None => 0,
        };

        let mut new_block = Block {
            index: self.chain.len() as u32,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            transactions,
            previous_hash,
            hash: 0,
        };

        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }
}

fn main() {
    // Create a new blockchain with difficulty level 4
    let mut blockchain = Blockchain::new(1);

    // Add some transactions and mine blocks
    blockchain.create_block(vec![Transaction {
        sender: "Alice".to_string(),
        recipient: "Bob".to_string(),
        amount: 10,
    }]);
    blockchain.create_block(vec![Transaction {
        sender: "Bob".to_string(),
        recipient: "Charlie".to_string(),
        amount: 5,
    }]);

    // Display the final blockchain
    println!("{:?}", blockchain);
}
