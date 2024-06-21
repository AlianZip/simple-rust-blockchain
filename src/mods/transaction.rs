use super::{block, blockchain::Blockchain, utxo::UTXO};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//Transaction
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Transaction {
    inputs: Vec<UTXO>,  //input UTXO
    outputs: Vec<UTXO>, //output UTXO
}



impl Transaction {
    pub(crate) fn new(amount: u64, recipient: String, from: String, blocks: Blockchain) -> Result<Transaction, &'static str> {
        if Transaction::check_utxo_from(from.clone(), blocks.clone()).is_empty() {
            return Err("No money");
        } else {
            let max_utxo = Transaction::max_in_hashmap(Transaction::check_utxo_from(from.clone(), blocks.clone()));
            let mut input_txid: String;
            let mut input_amount: u64;
            for (k, v) in max_utxo {
                input_txid = k;
                input_amount = v;
            }
            let new_transaction: Transaction = Transaction { inputs: vec![
                UTXO {txid: input_txid,
                    outputs_idx: Transaction::get_input_idx(from.clone(), blocks.clone()),
                    amount: input_amount,
                    recipient: recipient }],
                outputs: Transaction::generate_outputs_utxo(from.clone(), blocks.clone(), amount.clone(), recipient.clone()) };

            return Ok(new_transaction);
        }
    }

    fn check_utxo_from(from: String, blocks: Blockchain) -> HashMap<String, u64>{ //return txid: amount
        let mut utxos = HashMap::new();
        let mut del_utxos = Vec::new();

        for block in blocks.chain {
            for input in block.transaction.inputs {
                if input.recipient == from {
                    del_utxos.push(input.txid)
                }
            }
        }

        for block in blocks.chain {
            for output in block.transaction.outputs {
                if output.recipient == from {
                    if !del_utxos.contains(&output.txid) {
                        utxos.insert(output.txid, output.amount);
                    }
                }
            }
        }
        utxos
    }

    fn max_in_hashmap(hashmap: HashMap<String, u64>) -> HashMap<String, u64> {
        let mut max_score = 0;
        let mut max_key = "";
        
        for (key, value) in hashmap.iter() {
            if value > &max_score {
                max_score = *value;
                max_key = &*key;
            }
        }
        let mut new_map: HashMap<String, u64> = HashMap::new();
        new_map.insert(max_key.to_string(), max_score);
        new_map
    }

    fn generate_txid(from: String, blocks: Blockchain) -> String {
        let last_utxo = Transaction::get_last_utxo(from.clone(), blocks.clone());
        let output_utxo_string = format!("{}{}{}{}");
    }

    fn generate_outputs_utxo(from: String, blocks: Blockchain, amount: u64, recipient: String) -> Vec<UTXO> {

    }

    fn get_input_idx(from: String, blocks: Blockchain) -> u32 {

    }
}