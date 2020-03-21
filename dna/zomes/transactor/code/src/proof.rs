use crate::{attestation, attestation::TransactionRole, utils};
use hdk::{
    holochain_core_types::{chain_header::ChainHeader, signature::Signature},
    holochain_json_api::{error::JsonError, json::JsonString},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct TransactionCompletedProof {
    transaction_header: (ChainHeader, Signature),
    receiver_transaction_snapshot_proof: Signature,
}

pub fn get_transaction_proof(
    attestation_address: &Address,
) -> ZomeApiResult<TransactionCompletedProof> {
    let attestation = attestation::query_attestation(attestation_address)?;

    let proof = attestation
        .transaction_proof
        .ok_or(ZomeApiError::from(format!("Invalid attestation")))?;

    match proof.transaction_role {
        TransactionRole::Receiver { .. } => Err(ZomeApiError::from(format!("Invalid attestation"))),
        TransactionRole::Sender {
            receiver_transaction_snapshot_proof,
        } => {
            let chain_header = query_header(proof.transaction_header.0)?;

            Ok(TransactionCompletedProof {
                transaction_header: (chain_header, proof.transaction_header.1),
                receiver_transaction_snapshot_proof,
            })
        }
    }
}

pub fn query_header(header_address: Address) -> ZomeApiResult<ChainHeader> {
    let headers_with_entries = utils::query_all(String::from("*"))?;

    headers_with_entries
        .iter()
        .find(|header_with_entry| header_with_entry.0.address() == header_address)
        .map(|h| h.0)
        .ok_or(ZomeApiError::from(format!("Could not find header")))
}
