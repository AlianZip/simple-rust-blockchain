use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct UTXO {
    pub(crate) txid: String,
    pub(crate) outputs_idx: u32,
    pub(crate) amount: u64,
    pub(crate) recipient: String,
}