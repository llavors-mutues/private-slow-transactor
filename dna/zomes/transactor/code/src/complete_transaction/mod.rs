pub mod common;
pub mod receiver;
pub mod sender;

use hdk::{
    holochain_core_types::{chain_header::ChainHeader, signature::Signature},
    holochain_persistence_api::cas::content::Address,
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct AcceptOfferRequest {
    transaction_address: Address,
    approved_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CompleteTransactionRequest {
    pub chain_header: ChainHeader,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct CompleteTransactionResponse {
    chain_headers: Vec<ChainHeader>,
    signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SignAttestationRequest {
    chain_headers: Vec<ChainHeader>,
}
