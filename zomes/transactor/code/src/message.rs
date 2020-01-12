use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub transaction: Transaction,
    pub signature: String,
}
