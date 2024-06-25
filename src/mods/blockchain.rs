use super::{block::Block, transaction::Transaction, utxo::UTXO};
use chrono::Utc;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
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

    pub fn new_transaction(&mut self, amount: u64, recipient: String, from: String, public_key: PublicKey, secret_key: SecretKey) -> Result<&'static str, &'static str> {
        let index = self.chain.len() as u32; //get last index
        let transaction = Transaction::new(amount, recipient, from, self.clone(), public_key, secret_key); //new transaction in impl Transaction
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

    pub fn get_user_utxos(&self, user: String) -> Result<Vec<UTXO>, &'static str> {
        let all_utxo = Transaction::get_all_utxo(user, self.clone());

        if all_utxo.is_ok() {
            Ok(all_utxo.unwrap())
        } else {
            Err("No money")
        }
    }

    pub fn get_user_money(&self, user: String) -> Result<u64, &'static str> {
        let all_utxo = self.get_user_utxos(user);
        if all_utxo.is_ok() {
            let mut money = 0 as u64;
            for i in all_utxo.unwrap() {
                money += i.amount;
            }
            Ok(money)
        } else {
            Err("No money")
        }
    }

    pub fn generate_key_pair() -> (SecretKey, PublicKey) {
        let secp = Secp256k1::new();
        secp.generate_keypair(&mut OsRng)
    }
}