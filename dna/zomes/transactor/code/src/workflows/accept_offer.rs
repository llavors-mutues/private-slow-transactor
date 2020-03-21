use crate::utils::ParseableEntry;
use crate::{message::*, offer, transaction::Transaction, utils};
use hdk::holochain_core_types::{
    chain_header::ChainHeader,
    signature::{Provenance, Signature},
};
use super::TransactionCompletedProof;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct AcceptOfferRequest {
    transaction_address: Address,
    last_header_address: Address,
    receiver_transaction_snapshot_proof: Signature,
}

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
        receiver_transaction_snapshot_proof: signature,
    };

    let message = MessageBody::AcceptOffer(OfferMessage::Request(accept_offer_request));

    let response = match message {
        MessageBody::AcceptOffer(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "AcceptOffer response is not valid"
        ))),
    }?;

    match response {
        OfferResponse::OfferCompleted(proof) => complete_transaction(proof),
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
 * Builds and signs the snapshot proof for the given transaction and last_header
 */
fn create_snapshot_proof(
    transaction_address: &Address,
    last_header_address: &Address,
) -> ZomeApiResult<Signature> {
    let preimage = utils::snapshot_preimage(transaction_address, last_header_address);
    let signature = hdk::sign(preimage)?;
    Ok(Signature::from(signature))
}

pub fn complete_transaction(transaction_completed_proof: TransactionCompletedProof) -> ZomeApiResult<Address> {
    validate_transaction_header(chain_header, signature, &last_header_address, &transaction)?;

    offer::complete_offer(&transaction_address, &transaction.address()?)?;

    create_transaction_and_attestations(transaction)
}

/**
 * Validates that the chain header received from the sender of the transaction is appropriate for the offer that we sent
 */
fn validate_transaction_header(
    chain_header: ChainHeader,
    header_signature: Signature,
    last_header_address: &Address,
    transaction: &Transaction,
) -> ZomeApiResult<()> {
    if chain_header.entry_address().clone() != transaction.address()? {
        return Err(ZomeApiError::from(format!(
            "Received transaction address is not correct"
        )));
    }
    if chain_header.link().unwrap() != last_header_address.clone() {
        return Err(ZomeApiError::from(format!("Received chain header does not reference the last viewed header: there are new transactions")));
    }

    if transaction.address()? != chain_header.entry_address().clone() {
        return Err(ZomeApiError::from(format!(
            "Entry address in the header is not equal to the transaction address"
        )));
    }

    let chain_header_address = chain_header.address();
    hdk::verify_signature(
        Provenance::new(transaction.sender_address.clone(), header_signature),
        chain_header_address,
    )?;

    Ok(())
}

fn create_transaction_and_attestations(transaction: Transaction) -> ZomeApiResult<Address> {
    hdk::commit_entry(&transaction.entry())
}
