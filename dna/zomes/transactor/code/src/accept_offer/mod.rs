pub mod receiver;
pub mod sender;

use hdk::{
    holochain_core_types::signature::Signature, holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AcceptOfferRequest {
    transaction_address: Address,
    last_header_address: Address,
    receiver_transaction_snapshot_proof: Signature,
}
