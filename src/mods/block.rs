use super::transaction::Transaction;
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use serde_json::to_string;


//Block struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Block {
    index: u32,                 //index of block
    timestamp: i64,             //time when block create
    pub(crate) transaction: Transaction,   //Transaction
    hash: String,               //block hash
    preview_hash: String,       //preview block hash
}


impl Block {
    //create new block
    pub(crate) fn new(index: u32, transaction: Transaction, preview_hash: String) -> Block {
        let timestamp = Utc::now().timestamp(); //now
        
        Block {
            index,
            timestamp,
            transaction: transaction.clone(),
            hash: Block::get_hash(index, timestamp, transaction, preview_hash.clone()),
            preview_hash,
        }
    }

    pub(crate) fn hash_block(block: &Block) -> String {
        Block::get_hash(block.index.clone(), block.timestamp.clone(), block.transaction.clone(), block.preview_hash.clone())
    }

    fn get_hash(index: u32, timestamp: i64, transaction: Transaction, preview_hash: String) -> String {
        let transaction_string = to_string(&transaction).unwrap();
        let block_string = format!("{}{}{}{}", index, timestamp, transaction_string, preview_hash);
        let mut hasher = Sha256::new();
        hasher.input_str(&block_string);
        hasher.result_str()
    }


}