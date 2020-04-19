use super::{BalanceSnapshot, ChainSnapshot};
use crate::{
    attestation,
    message::{send_message, MessageBody, OfferMessage, OfferResponse},
    offer,
    offer::OfferState,
    transaction,
    transaction::Transaction,
};
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{prelude::*, AGENT_ADDRESS};
use holochain_entry_utils::HolochainEntry;

/*** One agent checks that the balance of the other is correct with the credit limit or in his opinion ***/

/**
 * Get the balance snapshot from the sender of the transaction
 * Then it returns offer balance, whether it's executable, and the last_header_address of the chain of that agent
 */
pub fn get_counterparty_balance(transaction_address: Address) -> ZomeApiResult<BalanceSnapshot> {
    let offer = offer::query_offer(&transaction_address)?;

    match offer.state {
        OfferState::Pending | OfferState::Approved { .. } => Ok(()),
        _ => Err(ZomeApiError::from(format!(
            "Offer is not pending: cannot get balance"
        ))),
    }?;

    let counterparty_address = match offer.transaction.debtor_address == AGENT_ADDRESS.clone() {
        true => offer.transaction.creditor_address.clone(),
        false => offer.transaction.debtor_address.clone(),
    };

    let chain_snapshot = request_chain_snapshot(&transaction_address, &counterparty_address)?;

    validate_snapshot_is_valid(&counterparty_address, &chain_snapshot)?;

    let mut transactions =
        transaction::get_transactions_from_chain_snapshot(chain_snapshot.snapshot);

    let balance = transaction::compute_balance(&counterparty_address, &transactions);

    // Add offer to the transaction list to verify that it is valid
    transactions.push(offer.transaction.clone());

    let executable =
        transaction::are_transactions_valid(&offer.transaction.debtor_address, &transactions)?;

    Ok(BalanceSnapshot {
        balance,
        executable,
        last_header_address: chain_snapshot.last_header_address,
    })
}

/**
 * Requests the transactions for the given offer_address from the counterparty agent, requesting their last header address for later validation
 */
fn request_chain_snapshot(
    transaction_address: &Address,
    counterparty_address: &Address,
) -> ZomeApiResult<ChainSnapshot> {
    let message = MessageBody::GetChainSnapshot(OfferMessage::Request(transaction_address.clone()));

    let result = send_message(counterparty_address.clone(), message)?;

    let response = match result {
        MessageBody::GetChainSnapshot(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "Error getting the chain snapshot for agent {}",
            counterparty_address
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
    chain_snapshot: &ChainSnapshot,
) -> ZomeApiResult<()> {
    validate_chain_snapshot(&chain_snapshot.snapshot)?;
    // Get the last attestation for the agent
    let (maybe_attestation, attestation_count) =
        attestation::get_latest_attestation_for(&agent_address)?;
    let transactions: Vec<(ChainHeader, Entry)> = chain_snapshot
        .snapshot
        .clone()
        .into_iter()
        .filter(|(_, entry)| Transaction::from_entry(&entry).is_some())
        .collect();

    if transactions.len() != attestation_count {
        return Err(ZomeApiError::from(String::from(
            "Number of attestations in the DHT does not match the recieved chain snapshot",
        )));
    }

    match (maybe_attestation, transactions.get(0)) {
        (Some(attestation), Some(transaction)) => {
            match attestation
                .header_addresses
                .contains(&transaction.0.address())
            {
                true => Ok(()),
                false => Err(ZomeApiError::from(String::from("Bad chain snapshot"))),
            }
        }
        (None, None) => Ok(()),
        _ => Err(ZomeApiError::from(String::from("Bad chain snapshot"))),
    }
}

/**
 * Validates that the given list of headers and entries is valid
 */
fn validate_chain_snapshot(chain_snapshot: &Vec<(ChainHeader, Entry)>) -> ZomeApiResult<()> {
    for i in 0..chain_snapshot.len() {
        let (chain_header, entry) = chain_snapshot[i].clone();
        if &entry.address() != chain_header.entry_address() {
            return Err(ZomeApiError::from(String::from("Bad chain header")));
        }

        if let Some((next_header, _)) = chain_snapshot.get(i + 1) {
            let next_header_address = next_header.address();
            if let Some(header_address) = chain_header.link() {
                if header_address == next_header_address {
                    return Ok(());
                }
            }

            return Err(ZomeApiError::from(String::from("Bad chain header")));
        }
    }

    Ok(())
}
