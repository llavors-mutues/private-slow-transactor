use crate::{attestation, attestation::TransactionRole, proof, utils};
use hdk::{
    holochain_core_types::{
        chain_header::ChainHeader,
        signature::{Provenance, Signature},
    },
    holochain_json_api::{error::JsonError, json::JsonString},
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct TransactionCompletedProof {
    pub transaction_header: (ChainHeader, Signature),
    pub receiver_snapshot_proof: Signature,
}

/**
 * Builds the snapshot preimage from the given transaction and last_header
 */
pub fn snapshot_proof_preimage(
    transaction_address: &Address,
    last_header_address: &Address,
) -> String {
    format!("{},{}", transaction_address, last_header_address)
}

/**
 * Gets the already existing proof for the given attestation address
 */
pub fn get_existing_transaction_proof(
    attestation_address: &Address,
) -> ZomeApiResult<TransactionCompletedProof> {
    let attestation = attestation::query_attestation(attestation_address)?;

    let proof = attestation
        .transaction_proof
        .ok_or(ZomeApiError::from(format!("Invalid attestation")))?;

    match proof.transaction_role {
        TransactionRole::Receiver { .. } => Err(ZomeApiError::from(format!("Invalid attestation"))),
        TransactionRole::Sender {
            receiver_snapshot_proof,
        } => {
            let chain_header = query_header(proof.transaction_header.0)?;

            Ok(TransactionCompletedProof {
                transaction_header: (chain_header, proof.transaction_header.1),
                receiver_snapshot_proof,
            })
        }
    }
}

/**
 * Returns the header identified with the given
 */
pub fn query_header(header_address: Address) -> ZomeApiResult<ChainHeader> {
    let headers_with_entries = utils::query_all(String::from("*"))?;

    headers_with_entries
        .iter()
        .find(|header_with_entry| header_with_entry.0.address() == header_address)
        .map(|h| h.0)
        .ok_or(ZomeApiError::from(format!("Could not find header")))
}

/**
 * Validates that the snapshot proof is valid for the given transaction and header for that transaction
 */
pub fn validate_snapshot_proof(
    receiver_address: &Address,
    transaction_address: &Address,
    last_header_address: &Address,
    snapshot_proof: &Signature,
) -> ZomeApiResult<()> {
    let preimage = proof::snapshot_proof_preimage(transaction_address, last_header_address);

    let provenance = Provenance::new(receiver_address.clone(), snapshot_proof.clone());

    match hdk::verify_signature(provenance, preimage)? {
        true => Ok(()),
        false => Err(ZomeApiError::from(format!("Signature is not valid"))),
    }
}
