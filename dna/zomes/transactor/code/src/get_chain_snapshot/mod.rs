use hdk::prelude::Entry;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::holochain_core_types::chain_header::ChainHeader;

pub mod receiver;
pub mod sender;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct BalanceSnapshot {
    balance: f64,
    executable: bool,
    last_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ChainSnapshot {
    pub snapshot: Vec<(ChainHeader, Entry)>,
    pub last_header_address: Address,
}
