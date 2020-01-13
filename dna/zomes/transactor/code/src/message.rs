use crate::transaction::Transaction;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::Entry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub transaction: Transaction,
    pub signature: String,
    pub chain_entries: Vec<(ChainHeader, Entry)>,
}
