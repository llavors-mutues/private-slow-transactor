use crate::{attestation, message::*, offer, transaction};
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::prelude::*;
use hdk::AGENT_ADDRESS;
use holochain_wasm_utils::api_serialization::{QueryArgsNames, QueryArgsOptions, QueryResult};

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct OfferBalance {
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
pub fn get_offer_balance(offer_address: Address) -> ZomeApiResult<OfferBalance> {
    let offer = offer::get_offer(&offer_address)?;

    match offer.state {
        offer::OfferState::Pending => Ok(()),
        _ => Err(ZomeApiError::from(format!(
            "Offer is canceled: cannot get balance"
        ))),
    }?;

    let transactions_snapshot = match offer.sender_address.clone() == AGENT_ADDRESS.clone() {
        true => get_my_transactions_snapshot(),
        false => {
            let snapshot = request_sender_transactions(&offer_address, &offer.sender_address)?;

            validate_snapshot_is_valid(&offer.sender_address, &snapshot)?;

            Ok(snapshot)
        }
    }?;

    let balance =
        transaction::compute_balance(&offer.sender_address, &transactions_snapshot.transactions);

    let next_transaction = offer.to_hypotetical_transaction();

    // Add offer to the transaction list to verify that it is valid
    let mut transactions_to_validate = transactions_snapshot.transactions;
    transactions_to_validate.push(next_transaction);

    let executable =
        transaction::are_transactions_valid(&offer.sender_address, &transactions_to_validate)?;

    Ok(OfferBalance {
        sender_balance: balance,
        executable,
        last_header_address: transactions_snapshot.last_header_address,
    })
}

/**
 * Requests the transactions for the given offer_address from the sender_address agent, requesting their last header address for later validation
 */
fn request_sender_transactions(
    offer_address: &Address,
    sender_address: &Address,
) -> ZomeApiResult<TransactionsSnapshot> {
    let message = MessageBody::GetTransactions(OfferMessage::Request(offer_address.clone()));

    let result = send_message(sender_address.clone(), message)?;

    let response = match result {
        MessageBody::GetTransactions(OfferMessage::Response(response)) => Ok(response),
        _ => Err(ZomeApiError::from(format!(
            "Error getting the transaction for agent {}",
            sender_address
        ))),
    }?;

    match response {
        OfferResponse::OfferPending(transactions_snapshot) => Ok(transactions_snapshot),
        OfferResponse::OfferNotPending => {
            offer::cancel_offer(offer_address)?;
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
    let dht_transactions_addresses =
        attestation::get_agent_transaction_addresses_from_dht(&agent_address)?;

    let snapshot_transaction_addresses: ZomeApiResult<Vec<Address>> = transaction_snapshot
        .transactions
        .iter()
        .map(|transaction| transaction.address())
        .collect();

    transaction::validate_transactions_against_attestations(
        &dht_transactions_addresses,
        &snapshot_transaction_addresses?,
    )
}

/*** Sender of the offer returns the list of private transactions if the offer is still pending ***/

/**
 * Get the transaction snapshot if the offer is still pending
 */
pub fn get_my_transactions_snapshot_for_offer(
    offer_address: Address,
) -> ZomeApiResult<OfferResponse<TransactionsSnapshot>> {
    let offer = offer::get_offer(&offer_address)?;

    if let offer::OfferState::Pending = offer.state {
        let transaction_snapshot = get_my_transactions_snapshot()?;

        return Ok(OfferResponse::OfferPending(transaction_snapshot));
    }

    Ok(OfferResponse::OfferNotPending)
}

/**
 * Get the list of transactions and the last header from the source chain
 */
fn get_my_transactions_snapshot() -> ZomeApiResult<TransactionsSnapshot> {
    let last_header = get_my_last_header()?;
    let transactions = transaction::get_all_my_transactions()?;

    Ok(TransactionsSnapshot {
        last_header_address: last_header.address(),
        transactions,
    })
}

/**
 * Gets the last header of my source chain
 */
fn get_my_last_header() -> ZomeApiResult<ChainHeader> {
    let options = QueryArgsOptions {
        start: 0,
        limit: 0,
        entries: false,
        headers: true,
    };
    let query_result = hdk::query_result(QueryArgsNames::from("*"), options)?;

    match query_result {
        QueryResult::Headers(headers) => headers
            .first()
            .ok_or(ZomeApiError::from(format!("Error getting the last header")))
            .map(|h| h.clone()),
        _ => Err(ZomeApiError::from(format!("Error getting the last header"))),
    }
}
