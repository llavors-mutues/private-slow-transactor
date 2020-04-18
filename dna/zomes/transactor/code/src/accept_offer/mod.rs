pub mod receiver;
pub mod sender;

use crate::{attestation, attestation::Attestation, offer};
use hdk::{
    holochain_core_types::signature::Signature, holochain_persistence_api::cas::content::Address,
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct AcceptOfferRequest {
    transaction_address: Address,
    last_header_address: Address,
    receiver_snapshot_proof: Signature,
}

/**
 * Gets the last attestation, updates it with the new one and completes the offer with its address
 */
pub fn complete_offer_and_update_attestation(new_attestation: Attestation) -> ZomeApiResult<()> {
    let proof = new_attestation
        .transaction_proof
        .clone()
        .ok_or(ZomeApiError::from(format!("Bad attestation")))?;
    let last_attestation = attestation::query_my_last_attestation()?;

    let attestation_address =
        hdk::update_entry(new_attestation.entry(), &last_attestation.address()?)?;

    offer::complete_offer(&proof.transaction_address, &attestation_address)?;

    Ok(())
}
