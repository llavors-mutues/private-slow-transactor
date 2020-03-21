use super::get_transaction_proof;
use crate::{attestation, message::*, offer, offer::OfferState, transaction, utils};
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct BalanceSnapshot {
    sender_balance: f64,
    executable: bool,
    last_header_address: Address,
}

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct TransactionsSnapshot {
    pub transactions: Vec<transaction::Transaction>,
    pub last_header_address: Address,
}

/*** Recipient of the offer checks that the balance of the sender is correct ***/

/**
 * Returns the offer balance, whether it's executable, and the last_header_address of the chain of the agent that made the offer
 */
pub fn get_sender_balance(transaction_address: Address) -> ZomeApiResult<BalanceSnapshot> {
    let offer = offer::query_offer(&transaction_address)?;

    match offer.state {
        offer::OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!(
            "Offer is canceled: cannot get balance"
        ))),
    }?;

    let transactions_snapshot =
        match offer.transaction.sender_address.clone() == AGENT_ADDRESS.clone() {
            true => get_my_transactions_snapshot(),
            false => {
                let snapshot = request_sender_transactions(
                    &transaction_address,
                    &offer.transaction.sender_address,
                )?;

                validate_snapshot_is_valid(&offer.transaction.sender_address, &snapshot)?;

                Ok(snapshot)
            }
        }?;

    let balance = transaction::compute_balance(
        &offer.transaction.sender_address,
        &transactions_snapshot.transactions,
    );

    // Add offer to the transaction list to verify that it is valid
    let mut transactions_to_validate = transactions_snapshot.transactions;
    transactions_to_validate.push(offer.transaction.clone());

    let executable = transaction::are_transactions_valid(
        &offer.transaction.sender_address,
        &transactions_to_validate,
    )?;

    Ok(BalanceSnapshot {
        sender_balance: balance,
        executable,
        last_header_address: transactions_snapshot.last_header_address,
    })
}

/**
 * Requests the transactions for the given offer_address from the sender_address agent, requesting their last header address for later validation
 */
fn request_sender_transactions(
    transaction_address: &Address,
    sender_address: &Address,
) -> ZomeApiResult<TransactionsSnapshot> {
    let message =
        MessageBody::GetTransactionsSnapshot(OfferMessage::Request(transaction_address.clone()));

    let result = send_message(sender_address.clone(), message)?;

    let response = match result {
        MessageBody::GetTransactionsSnapshot(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "Error getting the transaction for agent {}",
            sender_address
        ))),
    }?;

    match response {
        OfferResponse::OfferPending(transactions_snapshot) => Ok(transactions_snapshot),
        OfferResponse::OfferCanceled => {
            offer::cancel_offer(transaction_address)?;
            Err(ZomeApiError::from(format!("Offer was canceled")))
        }
    }
}

/**
 * Validate that the transaction snapshot received by the sender of the offer is valid with their attestations in the DHT
 */
fn validate_snapshot_is_valid(
    agent_address: &Address,
    transaction_snapshot: &TransactionsSnapshot,
) -> ZomeApiResult<()> {
    // Get transaction addresses for the agent
    let attestations = attestation::get_attestations_for_agent(&agent_address)?;

    attestation::validate_transactions_against_attestations(
        &attestations,
        &transaction_snapshot.transactions,
    )
}

/*** Sender of the offer returns the list of private transactions if the offer is still pending ***/

/**
 * Get the transaction snapshot if the offer is still pending
 */
pub fn get_transactions_snapshot(
    transaction_address: Address,
) -> ZomeApiResult<OfferResponse<TransactionsSnapshot>> {
    let offer = offer::query_offer(&transaction_address)?;

    match offer.state {
        OfferState::Pending => {
            let transaction_snapshot = get_my_transactions_snapshot()?;

            return Ok(OfferResponse::OfferPending(transaction_snapshot));
        }
        OfferState::Completed {
            attestation_address,
        } => get_transaction_proof(&attestation_address)
            .map(|proof| OfferResponse::OfferCompleted(proof)),
        OfferState::Canceled => Ok(OfferResponse::OfferCanceled),
    }
}

/**
 * Get the list of transactions and the last header from the source chain
 */
fn get_my_transactions_snapshot() -> ZomeApiResult<TransactionsSnapshot> {
    let last_header = get_my_last_header()?;
    let transactions = transaction::get_my_completed_transactions()?;

    Ok(TransactionsSnapshot {
        last_header_address: last_header.address(),
        transactions,
    })
}

/**
 * Gets the last header of my source chain
 */
fn get_my_last_header() -> ZomeApiResult<ChainHeader> {
    let headers_with_entries = utils::query_all(String::from("*"))?;

    headers_with_entries
        .first()
        .map(|h| h.0)
        .ok_or(ZomeApiError::from(format!("Could not find header")))
}
