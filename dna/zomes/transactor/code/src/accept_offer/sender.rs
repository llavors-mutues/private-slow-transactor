use super::{complete_offer_and_update_attestation, AcceptOfferRequest};
use crate::{
    attestation,
    attestation::Attestation,
    message::{send_message, MessageBody, OfferMessage, OfferResponse},
    offer, 
    transaction::Transaction,
    utils,
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
use holochain_entry_utils::HolochainEntry;

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

    offer::approve_offer(&transaction_address, &last_header_address)?;

    let accept_offer_request = AcceptOfferRequest {
        transaction_address: transaction_address.clone(),
        last_header_address: last_header_address.clone(),
        receiver_snapshot_proof: signature,
    };

    let message = MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request));

    let result = send_message(transaction.debtor_address.clone(), message)?;

    let response = match result {
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
    validate_proof(&transaction, &proof)?;

    let sender_attestation = Attestation::for_sender(
        &transaction.debtor_address.clone(),
        &proof.last_attestation_address,
        &transaction.address()?,
        &proof.transaction_header.0.address(),
        &proof.transaction_header.1,
        &proof.receiver_snapshot_proof,
    );

    let new_sender_attestation_address =
        hdk::update_entry(sender_attestation.entry(), &proof.last_attestation_address)?;
    let transaction_address = hdk::commit_entry(&transaction.clone().entry())?;
    let transaction_header = utils::get_my_last_header()?;

    let transaction_header_address = transaction_header.address();
    let header_signature = Signature::from(hdk::sign(transaction_header_address.clone())?);

    let last_attestation = attestation::query_my_last_attestation()?;
    let last_attestation_address = last_attestation.address()?;

    let receiver_attestation = Attestation::for_receiver(
        &AGENT_ADDRESS.clone(),
        &last_attestation_address,
        &transaction.address()?,
        &transaction_header_address,
        &header_signature,
        &new_sender_attestation_address,
    );

    complete_offer_and_update_attestation(receiver_attestation)?;

    Ok(transaction_address)
}

/**
 * Validates that the response proof is valid
 */
fn validate_proof(
    transaction: &Transaction,
    proof: &TransactionCompletedProof,
) -> ZomeApiResult<()> {
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

    Ok(())
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
        Provenance::new(transaction.debtor_address.clone(), header_signature.clone()),
        chain_header_address,
    )?;

    Ok(())
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
