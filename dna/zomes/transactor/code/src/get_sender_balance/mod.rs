use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use crate::transaction;

pub mod receiver;
pub mod sender;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct BalanceSnapshot {
    sender_balance: f64,
    executable: bool,
    last_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TransactionsSnapshot {
    pub transactions: Vec<transaction::Transaction>,
    pub last_header_address: Address,
}
