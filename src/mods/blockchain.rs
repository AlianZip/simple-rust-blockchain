use super::{block::Block, transaction::Transaction, utxo::UTXO};
use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Blockchain {
    pub(crate) chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![],
        }
    }

    pub fn new_transaction(&mut self, amount: u64, recipient: String, from: String) -> Result<&'static str, &'static str> {
        let index = self.chain.len() as u32; //get last index
        let transaction = Transaction::new(amount, recipient, from, self.clone()); //new transaction in impl Transaction
        let preview_hash = if let Some(preview_block) = self.chain.last() {
            Block::hash_block(preview_block) //if preview block exists
        } else {
            "XXX".to_string() //else preview block does not exists
        };
        if transaction.is_ok() {
            self.chain.push(Block::new(index, transaction.unwrap(), preview_hash)); //new block
            return Ok("Ok");
        } else {
            return Err("No money");
        }
    }

    pub fn check_user_utxos(&self, user: String) -> Result<Vec<UTXO>, &'static str> {
        let all_utxo = Transaction::get_all_utxo(user, self.clone());

        if all_utxo.is_ok() {
            Ok(all_utxo.unwrap())
        } else {
            Err("No money")
        }
    }

}