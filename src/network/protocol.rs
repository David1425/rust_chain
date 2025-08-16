#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
    Pong,
    GetBlocks(Vec<H256>), // Request blocks by hash
    Blocks(Vec<Block>),   // Send blocks in response
    Transaction(Transaction),
}