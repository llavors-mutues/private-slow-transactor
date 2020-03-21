use super::AcceptOfferRequest;
use crate::{
    message::{MessageBody, OfferMessage, OfferResponse},
    offer, proof,
    proof::TransactionCompletedProof,
    transaction::Transaction,
    utils::ParseableEntry,
};
use hdk::{
    holochain_core_types::{
        chain_header::ChainHeader,
        signature::{Provenance, Signature},
    },
    holochain_persistence_api::cas::content::Address,
    prelude::*,
    AGENT_ADDRESS,
};

/**
 * Accepts the offer, verifying that the source chain of the sender agent has not changed,
 * and creating the transaction privately
 */
pub fn accept_offer(
    transaction_address: Address,
    last_header_address: Address,
) -> ZomeApiResult<Address> {
    let offer = offer::query_offer(&transaction_address)?;

    let transaction = offer.transaction;

    let signature = create_snapshot_proof(&transaction_address, &last_header_address)?;

    let accept_offer_request = AcceptOfferRequest {
        transaction_address: transaction_address.clone(),
        last_header_address: last_header_address.clone(),
        receiver_snapshot_proof: signature,
    };

    let message = MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request));

    let response = match message {
        MessageBody::AcceptOffer(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "AcceptOffer response is not valid"
        ))),
    }?;

    match response {
        OfferResponse::OfferCompleted(proof) => complete_transaction(transaction, proof),
        OfferResponse::OfferPending(()) => {
            Err(ZomeApiError::from(format!("Offer is still pending?")))
        }
        OfferResponse::OfferCanceled => {
            offer::cancel_offer(&transaction_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
}

/**
 * Validate the received proof from the sender of the transaction and execute it
 */
pub fn complete_transaction(
    transaction: Transaction,
    proof: TransactionCompletedProof,
) -> ZomeApiResult<Address> {
    let last_header_address =
        proof
            .transaction_header
            .0
            .link()
            .ok_or(ZomeApiError::from(format!(
                "Bad chain header: no last header present"
            )))?;

    proof::validate_snapshot_proof(
        &AGENT_ADDRESS.clone(),
        &transaction.address()?,
        &last_header_address,
        &proof.receiver_snapshot_proof,
    )?;

    validate_transaction_header(
        &transaction,
        &proof.transaction_header.0,
        &proof.transaction_header.1,
    )?;

    create_transaction_and_attestations(transaction)?;

    offer::complete_offer(&transaction.address()?)?;
}

/**
 * Validates that the chain header received from the sender of the transaction is appropriate for the offer that we sent
 */
fn validate_transaction_header(
    transaction: &Transaction,
    chain_header: &ChainHeader,
    header_signature: &Signature,
) -> ZomeApiResult<()> {
    if chain_header.entry_address().clone() != transaction.address()? {
        return Err(ZomeApiError::from(format!(
            "Received transaction address is not correct"
        )));
    }
    if transaction.address()? != chain_header.entry_address().clone() {
        return Err(ZomeApiError::from(format!(
            "Entry address in the header is not equal to the transaction address"
        )));
    }

    let chain_header_address = chain_header.address();
    hdk::verify_signature(
        Provenance::new(transaction.sender_address.clone(), header_signature.clone()),
        chain_header_address,
    )?;

    Ok(())
}

/**
 * After validation, create the transaction entry and the attestations for both the sender and the receiver
 */
fn create_transaction_and_attestations(transaction: Transaction) -> ZomeApiResult<Address> {
    hdk::commit_entry(&transaction.entry())
}

/**
 * Builds and signs the snapshot proof for the given transaction and last_header
 */
fn create_snapshot_proof(
    transaction_address: &Address,
    last_header_address: &Address,
) -> ZomeApiResult<Signature> {
    let preimage = proof::snapshot_proof_preimage(transaction_address, last_header_address);
    let signature = hdk::sign(preimage)?;
    Ok(Signature::from(signature))
}
