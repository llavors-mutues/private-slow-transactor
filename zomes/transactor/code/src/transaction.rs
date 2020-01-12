use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{holochain_core_types::entry::Entry, holochain_persistence_api::cas::content::Address};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Transaction {
    pub sender_address: Address,
    pub receiver_address: Address,
    pub timestamp: usize,
    pub amount: usize,
}

pub fn transaction_entry(transaction: &Transaction) -> Entry {
    Entry::App("transaction".into(), transaction.clone().into())
}
