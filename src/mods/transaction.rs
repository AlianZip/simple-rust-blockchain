use super::{blockchain::Blockchain, utxo::UTXO};
use chrono::Utc;
use crypto::{digest::Digest, sha2::Sha256};
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use secp256k1::hashes::{sha256, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;


//Transaction
#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Transaction {
    inputs: Vec<UTXO>,  //input UTXOs
    outputs: Vec<UTXO>, //output UTXOs
    public_key: String,
    signature: String,
}



impl Transaction {
    pub(crate) fn new(amount: u64, recipient: String, from: String, blocks: Blockchain, public_key: PublicKey, secret_key: SecretKey) -> Result<Transaction, &'static str> {
        if Transaction::get_max_utxo(from.clone(), blocks.clone()).is_err() && !(from == "system") {
            return Err("No money");
        } else {
            let inputs = Transaction::generate_input_utxos(from.clone(), blocks.clone(), amount.clone());
            if inputs.is_ok() {
                let outputs = Transaction::generate_outputs_utxos(from, blocks, amount, recipient);
                let signature = Transaction::signature_transaction(inputs.clone().unwrap(), outputs.clone(), secret_key);
                let new_transaction: Transaction = Transaction {
                    inputs: inputs.unwrap(),
                    outputs: outputs,
                    public_key: public_key.to_string(),
                    signature: signature.to_string()};

                return Ok(new_transaction);
            } else {
                Err("No money")
            }
        }
    }

    pub fn get_all_utxo(from: String, blocks: Blockchain) -> Result<Vec<UTXO>, &'static str> {
        let mut utxos = Vec::new();
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
                        utxos.push(output);
                    }
                }
            }
        }

        if utxos.is_empty() {
            Err("No money")
        } else {
            Ok(utxos)
        }
    }

    fn get_max_utxo(from: String, blocks: Blockchain) -> Result<UTXO, &'static str> { //return txid: amount
        if from == "system" {
            return Ok(UTXO {
                txid: String::new(),
                outputs_idx: 0,
                amount: 0,
                recipient: String::new(),
            });
        }
        let mut utxos = HashMap::new();
        let mut del_utxos = Vec::new();

        for block in blocks.chain.clone() {
            for input in block.clone().transaction.inputs {
                if input.recipient == from {
                    del_utxos.push(input.clone().txid)
                }
            }
        }
        

        for block in blocks.chain.clone() {
            for output in block.transaction.outputs {
                if output.recipient == from {
                    if !(del_utxos.contains(&output.txid)) {
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
                    if output.recipient == from{
                        max_utxo = output;
                        return Ok(max_utxo);
                    }
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

    fn generate_txid(input_utxo: UTXO, outputs_idx: u32, timestamp: String) -> String {
        let output_utxo_string = format!("{}{}{}{}{}",
            input_utxo.txid,
            outputs_idx,
            input_utxo.amount,
            input_utxo.recipient,
            timestamp);
        let mut hasher = Sha256::new();
        hasher.input_str(&output_utxo_string);
        hasher.result_str()
    }

    fn generate_outputs_utxos(from: String, blocks: Blockchain, amount: u64, recipient: String) -> Vec<UTXO> {
        let max_utxo = Transaction::get_max_utxo(from.clone(), blocks.clone());
        let remains: u64;
        if from == "system".to_string() {
            remains = 0;
        } else {
            remains = max_utxo.clone().unwrap().amount - amount;
        }
        let timestamp = Utc::now().timestamp_millis();
        let outputs_utxos = vec![
            UTXO {
                txid: Transaction::generate_txid(max_utxo.clone().unwrap(), max_utxo.clone().unwrap().outputs_idx+1, timestamp.to_string()),
                outputs_idx: max_utxo.clone().unwrap().outputs_idx+1,
                amount: amount,
                recipient: recipient,
            },
            UTXO {
                txid: Transaction::generate_txid(max_utxo.clone().unwrap(), max_utxo.clone().unwrap().outputs_idx+2, timestamp.to_string()),
                outputs_idx: max_utxo.clone().unwrap().outputs_idx+2,
                amount: remains,
                recipient: from.clone(),
            }];
            outputs_utxos
    }

    fn generate_input_utxos(from: String, blocks: Blockchain, amount: u64, ) -> Result<Vec<UTXO>, &'static str> {
        let max_utxo = Transaction::get_max_utxo(from.clone(), blocks);
        if !(from == "system") {
            if max_utxo.clone().unwrap().amount < amount {
                Err("No money")
            } else {
                Ok(vec![max_utxo.unwrap()])
            }
        } else {
            let timestamp = Utc::now().timestamp_millis();
            Ok(vec![
                UTXO {
                    txid: Transaction::generate_txid(max_utxo.clone().unwrap(), max_utxo.clone().unwrap().outputs_idx+1, timestamp.to_string()),
                    outputs_idx: 0,
                    amount: 0,
                    recipient: "system".to_string(),
                }
            ])
        }
    }

    fn signature_transaction(inputs: Vec<UTXO>, outputs: Vec<UTXO>, secret_key: SecretKey) -> Signature {
        let string_of_vectors = format!("{:?}{:?}", inputs, outputs); 

        let secp = Secp256k1::new();

        let digest = sha256::Hash::hash(string_of_vectors.as_bytes()); 
        let message = Message::from_digest(*digest.as_byte_array());

        secp.sign_ecdsa(&message, &secret_key)
    }

    fn verify_transaction(inputs: Vec<UTXO>, outputs: Vec<UTXO>, sig: String, pk: PublicKey) -> bool {
        let secp = Secp256k1::new();
        let string_of_vectors = format!("{:?}{:?}", inputs, outputs);
        let digest = sha256::Hash::hash(string_of_vectors.as_bytes()); 
        let message = Message::from_digest(*digest.as_byte_array());
        secp.verify_ecdsa(&message, &Signature::from_str(&sig).unwrap(), &pk).is_ok()
    }
}