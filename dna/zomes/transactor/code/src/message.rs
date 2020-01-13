use crate::transaction::Transaction;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageBody {
    pub transaction: Transaction,
    pub signature: String,
    pub old_transactions: Vec<Transaction>,
}
