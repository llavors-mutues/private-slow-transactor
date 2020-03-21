use super::TransactionsSnapshot;
use crate::{message::OfferResponse, offer, offer::OfferState, proof, transaction, utils};
use hdk::{prelude::*, AGENT_ADDRESS};

/*** Sender of the offer returns the list of private transactions if the offer is still pending ***/

/**
 * Get the transaction snapshot if the offer is still pending
 */
pub fn get_transactions_snapshot(
    transaction_address: Address,
) -> ZomeApiResult<OfferResponse<TransactionsSnapshot>> {
    let offer = offer::query_offer(&transaction_address)?;

    if offer.transaction.sender_address != AGENT_ADDRESS.clone() {
        return Err(ZomeApiError::from(format!(
            "I'm not the sender of the given transaction"
        )));
    }

    match offer.state {
        OfferState::Pending => {
            let transaction_snapshot = get_my_transactions_snapshot()?;

            return Ok(OfferResponse::OfferPending(transaction_snapshot));
        }
        OfferState::Completed {
            attestation_address,
        } => proof::get_existing_transaction_proof(&attestation_address)
            .map(|proof| OfferResponse::OfferCompleted(proof)),
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
    }
}

/**
 * Get the list of transactions and the last header from the source chain
 */
pub fn get_my_transactions_snapshot() -> ZomeApiResult<TransactionsSnapshot> {
    let last_header = utils::get_my_last_header()?;
    let transactions = transaction::get_my_completed_transactions()?;

    Ok(TransactionsSnapshot {
        last_header_address: last_header.address(),
        transactions,
    })
}
