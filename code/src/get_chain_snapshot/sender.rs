use super::{ChainSnapshot, CounterpartySnapshot};
use crate::{
    attestation,
    message::{send_message, MessageBody, OfferMessage, OfferResponse},
    offer,
    offer::OfferState,
    transaction,
    transaction::Transaction,
};
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

/**
 * Get the balance snapshot from the sender of the transaction
 * Then it returns offer balance, whether it's executable, and the last_header_address of the chain of that agent
 */
pub fn get_counterparty_snapshot(
    transaction_address: Address,
) -> ZomeApiResult<CounterpartySnapshot> {
    let offer = offer::query_offer(&transaction_address)?;

    match offer.state {
        OfferState::Pending | OfferState::Approved { .. } => Ok(()),
        _ => Err(ZomeApiError::from(format!(
            "Offer is not pending: cannot get balance"
        ))),
    }?;

    let counterparty_address = transaction::get_counterparty(&offer.transaction);

    let chain_snapshot = request_chain_snapshot(&transaction_address, &counterparty_address)?;

    let mut transactions =
        transaction::get_transactions_from_chain_snapshot(chain_snapshot.snapshot.clone());

    let (valid, invalid_reason) =
        match validate_snapshot_is_valid(&counterparty_address, &chain_snapshot) {
            Ok(()) => {
                let result =
                    transaction::are_transactions_valid(&counterparty_address, &transactions);
                match result {
                    Ok(true) => (true, None),
                    Ok(false) => (
                        false,
                        Some(format!("Agent's balance is beyond the credit limit")),
                    ),
                    Err(err) => (false, Some(format!("{:?}", err))),
                }
            }
            Err(err) => (false, Some(format!("{:?}", err))),
        };

    let balance = transaction::compute_balance(&counterparty_address, &transactions);

    // Add offer to the transaction list to verify that it is valid
    transactions.push(offer.transaction.clone());

    let executable = valid
        && transaction::are_transactions_valid(&offer.transaction.debtor_address, &transactions)?;

    Ok(CounterpartySnapshot {
        balance,
        executable,
        valid,
        invalid_reason,
        last_header_address: chain_snapshot.snapshot[0].0.address(),
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

    let result = match send_message(counterparty_address.clone(), message) {
        Ok(r) => Ok(r),
        Err(_) => Err(ZomeApiError::from(String::from(
            "Counterparty is offline at the moment, could not get their chain snapshot",
        ))),
    }?;

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
        OfferResponse::OfferCompleted(_) => {
            Err(ZomeApiError::from(format!("Offer is already completed")))
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
    for (i, (chain_header, entry)) in chain_snapshot.iter().enumerate() {
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
