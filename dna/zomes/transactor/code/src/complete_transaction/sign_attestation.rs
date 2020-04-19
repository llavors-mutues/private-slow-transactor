use super::common::{
    validate_counterparty_header, validate_last_header_still_unchanged,
    validate_transaction_headers,
};
use crate::{
    attestation::Attestation, message::OfferResponse, offer, offer::OfferState, transaction,
};
use hdk::{
    holochain_core_types::{chain_header::ChainHeader, signature::Signature},
    prelude::*,
    AGENT_ADDRESS,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SignAttestationRequest {
    pub chain_headers: Vec<ChainHeader>,
}

/**
 * Verifies that offer is approved, and then begins the complete transaction workflow
 */
pub fn receive_sign_attestation_request(
    sender_address: Address,
    sign_attestation_request: SignAttestationRequest,
) -> ZomeApiResult<OfferResponse<Signature>> {
    let transaction_address = sign_attestation_request.chain_headers[0].entry_address();
    let offer = offer::query_offer(&transaction_address)?;

    let transaction = offer.transaction;

    let counterparty = transaction::get_counterparty(&transaction);

    if sender_address != counterparty {
        return Err(ZomeApiError::from(String::from(
            "Agent sending the message is not the counterparty for this transaction",
        )));
    }
    match offer.state {
        OfferState::Approved {
            approved_header_address,
        } => {
            let signature =
                handle_sign_attestation(sign_attestation_request, approved_header_address)?;

            Ok(OfferResponse::OfferPending(signature))
        }
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
        _ => Err(ZomeApiError::from(format!(
            "Offer for transaction {} has not been approved",
            transaction_address
        ))),
    }
}

/**
 * Assumes that the offer is still pending
 *
 * 1. Check that my header has not moved
 * 2. Check that the transaction headers are valid
 * 3. Check that the counterparty's header is valid
 * 4. Build and sign the attestation entry
 */
pub fn handle_sign_attestation(
    sign_attestation_request: SignAttestationRequest,
    approved_header_address: Option<Address>,
) -> ZomeApiResult<Signature> {
    let my_header = sign_attestation_request
        .chain_headers
        .iter()
        .find(|h| h.provenances()[0].source() == AGENT_ADDRESS.clone())
        .ok_or(ZomeApiError::from(String::from(
            "Could not find my transaction header",
        )))?;

    let previous_header_address = my_header
        .link()
        .ok_or(ZomeApiError::from(String::from("Bad header")))?;

    validate_last_header_still_unchanged(previous_header_address)?;

    validate_transaction_headers(&sign_attestation_request.chain_headers)?;

    let counterparty_header = sign_attestation_request
        .chain_headers
        .iter()
        .find(|h| h.provenances()[0].source() != AGENT_ADDRESS.clone())
        .ok_or(ZomeApiError::from(String::from(
            "Could not find the transaction header for my counterparty",
        )))?;

    let transaction_address = sign_attestation_request.chain_headers[0].entry_address();
    let offer = offer::query_offer(&transaction_address)?;

    validate_counterparty_header(
        &counterparty_header,
        &offer.transaction,
        &approved_header_address,
    )?;

    let attestation = Attestation::from_headers(&sign_attestation_request.chain_headers);

    let signature = hdk::sign(attestation.address()?)?;

    Ok(Signature::from(signature))
}
