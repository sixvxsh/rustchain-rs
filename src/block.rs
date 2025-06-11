extern crate bincode;
use bincode::{Decode,Encode};
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    // Functions related to the Block type can be implemented here


}

pub fn new_block(pre_block_hash: String, transactions: &[transactions], height: usize) -> Block {
    let mut block = Block {
        timestamp: crate::current_timestamp(),
        pre_block_hash,
        transactions: transactions.to_vec(),
        height,
        nonce: 0,
        hash: String::new(),
    };
    let pow = ProofOfWork::new_proof_of_work(block.clone());
    let (nonce, hash) = pow.run();
    block.nonce = nonce;
    block.hash = hash;
    return block;

}



