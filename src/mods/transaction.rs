use super::{blockchain::Blockchain, utxo::UTXO};
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//Transaction
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Transaction {
    inputs: Vec<UTXO>,  //input UTXOs
    outputs: Vec<UTXO>, //output UTXOs
}



impl Transaction {
    pub(crate) fn new(amount: u64, recipient: String, from: String, blocks: Blockchain) -> Result<Transaction, &'static str> {
        if Transaction::get_max_utxo(from.clone(), blocks.clone()).is_err() {
            return Err("No money");
        } else {
            let inputs = Transaction::generate_input_utxos(from.clone(), blocks.clone(), amount.clone()); 
            
            if inputs.is_ok() {
                let new_transaction: Transaction = Transaction {
                    inputs: inputs.unwrap(),
                    outputs: Transaction::generate_outputs_utxos(from, blocks, amount, recipient) };

                return Ok(new_transaction);
            } else {
                Err("No money")
            }
        }
    }

    fn get_max_utxo(from: String, blocks: Blockchain) -> Result<UTXO, &'static str>{ //return txid: amount
        let mut utxos = HashMap::new();
        let mut del_utxos = Vec::new();

        for block in blocks.chain.clone() {
            for input in block.transaction.inputs {
                if input.recipient == from {
                    del_utxos.push(input.txid)
                }
            }
        }

        for block in blocks.chain.clone() {
            for output in block.transaction.outputs {
                if output.recipient == from {
                    if !del_utxos.contains(&output.txid) {
                        utxos.insert(output.txid, output.amount);
                    }
                }
            }
        }
        
        let max_utxo_hashmap = Transaction::max_in_hashmap(utxos);

        let mut output_txid_max_utxo = String::new();
        for (k, _) in max_utxo_hashmap {
            output_txid_max_utxo = k;
        };
        
        let max_utxo: UTXO;

        for block in blocks.chain {
            for output in block.transaction.outputs {
                if output.txid == output_txid_max_utxo {
                    max_utxo = output;
                    return Ok(max_utxo);
                }
            }
        }
        
        Err("No money")
        
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

    fn generate_txid(from: String, input_utxo: UTXO, outputs_idx: u32) -> String {
        let output_utxo_string = format!("{}{}{}{}",
            input_utxo.txid,
            outputs_idx,
            input_utxo.amount,
            input_utxo.recipient);
        let mut hasher = Sha256::new();
        hasher.input_str(&output_utxo_string);
        hasher.result_str()
    }

    fn generate_outputs_utxos(from: String, blocks: Blockchain, amount: u64, recipient: String) -> Vec<UTXO> {
        let max_utxo = Transaction::get_max_utxo(from.clone(), blocks.clone());
        let remains = max_utxo.clone().unwrap().amount - amount;
        vec![
        UTXO {
            txid: Transaction::generate_txid(from.clone(), max_utxo.clone().unwrap(), 0),
            outputs_idx: 0,
            amount: amount,
            recipient: recipient,
        },
        UTXO {
            txid: Transaction::generate_txid(from.clone(), max_utxo.unwrap(), 1),
            outputs_idx: 1,
            amount: remains,
            recipient: from,
        }]
    }

    fn generate_input_utxos(from: String, blocks: Blockchain, amount: u64) -> Result<Vec<UTXO>, &'static str> {
        let max_utxo = Transaction::get_max_utxo(from, blocks);
        if max_utxo.clone().unwrap().amount < amount {
            Err("No money")
        } else {
            Ok(vec![max_utxo.unwrap()])
        }
    }
}