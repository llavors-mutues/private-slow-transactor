use super::{complete_offer_and_update_attestation, AcceptOfferRequest};
use crate::{
    attestation, attestation::Attestation, message::OfferResponse, offer, offer::OfferState, proof,
    proof::TransactionCompletedProof, transaction::Transaction, utils, utils::ParseableEntry,
};
use hdk::{holochain_core_types::signature::Signature, prelude::*, AGENT_ADDRESS};

/**
 * Process accept offer request, creating the transaction, updating the offer and updating the attestation
 */
pub fn receive_accept_offer(sender_address: Address, request: AcceptOfferRequest) -> ZomeApiResult<OfferResponse<()>> {
    let transaction_address = request.transaction_address.clone();
    let offer = offer::query_offer(&transaction_address)?;

    if offer.transaction.receiver_address != sender_address {
        return Err(ZomeApiError::from(format!(
            "The sender of the message is not the receiver of the transaction"
        )));
    }

    match offer.state {
        OfferState::Pending => create_transaction_and_attestation(offer.transaction, request)
            .map(|proof| OfferResponse::OfferCompleted(proof)),
        OfferState::Completed {
            attestation_address,
        } => proof::get_existing_transaction_proof(&attestation_address)
            .map(|proof| OfferResponse::OfferCompleted(proof)),
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
    }
}

/**
 * Validate signatures and snapshot proof, create the transaction and my attestation, and return TransactionCompletedProof
 */
pub fn create_transaction_and_attestation(
    transaction: Transaction,
    request: AcceptOfferRequest,
) -> ZomeApiResult<TransactionCompletedProof> {
    validate_request(&transaction, &request)?;
    let transaction_address = hdk::commit_entry(&transaction.entry())?;

    let transaction_header = utils::get_my_last_header()?;

    let transaction_header_address = transaction_header.address();

    attestation::create_initial_attestation()?;

    let header_signature = Signature::from(hdk::sign(transaction_header_address.clone())?);
    let last_attestation = attestation::query_my_last_attestation()?;
    let last_attestation_address = last_attestation.address()?;

    let attestation = Attestation::for_sender(
        &AGENT_ADDRESS.clone(),
        &last_attestation_address,
        &transaction_address,
        &transaction_header_address,
        &header_signature,
        &request.receiver_snapshot_proof,
    );

    complete_offer_and_update_attestation(attestation)?;

    Ok(TransactionCompletedProof {
        transaction_header: (transaction_header, header_signature),
        last_attestation_address,
        receiver_snapshot_proof: request.receiver_snapshot_proof,
    })
}

/**
 * Validate that the transaction
 */
fn validate_request(transaction: &Transaction, request: &AcceptOfferRequest) -> ZomeApiResult<()> {
    if transaction.sender_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!(
            "I'm not the sender of the given transaction"
        )));
    }

    let last_chain_header = utils::get_my_last_header()?;

    if last_chain_header.address() != request.last_header_address {
        return Err(ZomeApiError::from(format!(
            "Last header has moved: try again"
        )));
    }

    proof::validate_snapshot_proof(
        &transaction.receiver_address,
        &transaction.address()?,
        &request.last_header_address,
        &request.receiver_snapshot_proof,
    )?;

    Ok(())
}
