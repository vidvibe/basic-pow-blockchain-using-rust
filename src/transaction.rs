// Define a basic transaction structure
#[derive(Debug, Hash)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u32,
}
